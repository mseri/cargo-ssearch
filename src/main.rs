#![allow(unused_must_use)]
extern crate docopt;
extern crate reqwest;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate term;
extern crate url;

use std::env::args;
use std::error::Error;
use std::process;
use std::str;

//use reqwest::reqwest;
use docopt::Docopt;
use url::percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};

#[macro_use]
mod macros;

const USAGE: &'static str = "
Scrutch - Crates Search

Usage:
  scrutch [--info] <query>
  scrutch (-h | --help)
  scrutch (-v | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
  --info        Show complete details of the crates.
";

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

fn query_crates_io(query: &str) -> Result<Vec<EncodableCrate>, Box<Error>> {
    // TODO: think about implementing page and per_page via an option,
    //       use Url::parse_with_params
    let url = format!(
        "https://crates.io/api/v1/crates?q={}", //&page=1&per_page=10",
        utf8_percent_encode(query, QUERY_ENCODE_SET)
    );
    let body = reqwest::get(&url)?.text()?;
    let data: Response = serde_json::from_str(&body)?;
    Ok(data.crates)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args().into_iter()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut t = term::stdout().unwrap();

    // TODO: Add decoding of updated_at and allow to use it for sorting
    let mut crates = query_crates_io(&args.arg_query).unwrap_or_else(|e| {
        p_red!(t, "[error]: {}\n", e.description());
        process::exit(1)
    });
    crates.sort_by(|c1, c2| c2.downloads.cmp(&c1.downloads));

    p_white!(
        t,
        "scrutch: {} crates found with query: \"{}\"\n\n",
        crates.len(),
        args.arg_query
    );
    for cr in crates {
        show_crate(&mut t, &cr, args.flag_info);
    }
    t.reset().unwrap();
}

fn show_crate(t: &mut Box<term::StdoutTerminal>, cr: &EncodableCrate, info: bool) {
    // TODO: make it more DRY
    if info {
        p_green!(t, "{}", cr.name);
    } else {
        p_green!(t, "{:<20}", cr.name);
    }

    // TODO: pad and align output more sensibly
    p_white!(
        t,
        " = \"{}\"\t(downloads: {})\n",
        cr.max_version,
        cr.downloads
    );

    if info {
        if_some!(
            &cr.description,
            p_yellow!(t, " -> {}\n", &cr.description.clone().unwrap().trim())
        );
        if_some!(
            &cr.documentation,
            p_white!(t, "    docs: {}\n", &cr.documentation.clone().unwrap())
        );
        if_some!(
            &cr.homepage,
            p_white!(t, "    home: {}\n", &cr.homepage.clone().unwrap())
        );
        p_white!(t, "\n");
    }
}
