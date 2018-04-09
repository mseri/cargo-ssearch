#![allow(unused_must_use)]
extern crate pad;
#[macro_use]
extern crate quicli;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate term;

use pad::PadStr;
use reqwest::Url;

use std::process;
use std::str;

use quicli::prelude::*;

#[macro_use]
mod macros;

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo")]
enum Cli {
    #[structopt(name = "ssearch", about = "cargo search on steroids")]
    Ssearch {
        /// how many packages to display
        #[structopt(long = "limit", short = "l", default_value = "10")]
        limit: usize,
        /// the crates.io search result page to display
        #[structopt(long = "page", default_value = "1")]
        page: usize,
        /// quiet output, display only crate, version and downloads
        #[structopt(long = "quiet", short = "q")]
        quiet: bool,
        /// sort by recent downloads instead of overall downloads
        #[structopt(long = "recent", short = "r")]
        recent: bool,
        /// query string for crates.io
        query: String,
    },
}

#[derive(Debug, Deserialize)]
struct Args {
    flag_info: bool,
    arg_query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    crates: Vec<EncodableCrate>,
    meta: Meta,
}

// structs from crates.io backend

#[derive(Debug, Serialize, Deserialize)]
struct EncodableCrate {
    id: String,
    name: String,
    updated_at: String,
    versions: Option<Vec<i32>>,
    created_at: String,
    downloads: i32,
    max_version: String,
    description: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    keywords: Option<Vec<String>>,
    license: Option<String>,
    repository: Option<String>,
    links: CrateLinks,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrateLinks {
    version_downloads: String,
    versions: Option<String>,
    owners: Option<String>,
    reverse_dependencies: String,
}

fn query_crates_io(
    query: &str,
    page: usize,
    per_page: usize,
    recent: bool,
) -> Result<(i32, Vec<EncodableCrate>)> {
    let sort = if recent {
        "recent-downloads"
    } else {
        "downloads"
    };
    let url = Url::parse_with_params(
        "https://crates.io/api/v1/crates",
        &[
            ("q", query),
            ("page", &page.to_string()),
            ("per_page", &per_page.to_string()),
            ("sort", &sort),
        ],
    )?;
    let body = reqwest::get(url)?.text()?;
    let data: Response = serde_json::from_str(&body)?;
    Ok((data.meta.total, data.crates))
}

fn show_crate(t: &mut Box<term::StdoutTerminal>, cr: &EncodableCrate, quiet: bool, max_len: usize) {
    p_green!(t, "{}", cr.name.pad_to_width(max_len));
    p_white!(
        t,
        " = \"{}\"    \t(downloads: {})\n",
        cr.max_version,
        cr.downloads
    );

    if !quiet {
        cr.description
            .as_ref()
            .map(|description| p_yellow!(t, " -> {}\n", description.clone().trim()));
        cr.documentation
            .as_ref()
            .map(|documentation| p_white!(t, "    docs: {}\n", documentation));
        cr.homepage
            .as_ref()
            .map(|homepage| p_white!(t, "    home: {}\n", homepage));
        p_white!(t, "\n");
    }
}

main!(|args: Cli| {
    let Cli::Ssearch {
        query,
        page,
        limit,
        quiet,
        recent,
    } = args;

    let mut t = term::stdout().unwrap();

    // TODO: Add decoding of updated_at and allow to use it for sorting
    let (total, crates) = query_crates_io(&query, page, limit, recent).unwrap_or_else(|e| {
        p_red!(t, "[error]: {}.\n", e);
        t.reset().unwrap();
        process::exit(1)
    });

    p_white!(
        t,
        "Displaying {} crates from page {} out of the {} found.\n\n",
        crates.len(),
        page,
        total,
    );
    let max_len = (&crates).iter().map(|ref cr| cr.name.len()).max().unwrap();
    for cr in &crates {
        show_crate(&mut t, &cr, quiet, max_len);
    }

    t.reset().unwrap();
});
