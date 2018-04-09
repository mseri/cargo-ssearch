#![allow(unused_must_use)]
extern crate docopt;
extern crate pad;
extern crate reqwest;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate term;

use std::env::args;
use std::error::Error;
use pad::PadStr;
use std::process;
use std::str;

use docopt::Docopt;
use reqwest::Url;

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

fn query_crates_io(
    query: &str,
    page: i64,
    per_page: i64,
) -> Result<(i32, Vec<EncodableCrate>), Box<Error>> {
    let url = Url::parse_with_params(
        "https://crates.io/api/v1/crates",
        &[
            ("q", query),
            ("page", &page.to_string()),
            ("per_page", &per_page.to_string()),
        ],
    )?;
    let body = reqwest::get(url)?.text()?;
    let data: Response = serde_json::from_str(&body)?;
    Ok((data.meta.total, data.crates))
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args().into_iter()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut t = term::stdout().unwrap();

    // TODO: Add decoding of updated_at and allow to use it for sorting
    let (total, mut crates) = query_crates_io(&args.arg_query, 1, 10).unwrap_or_else(|e| {
        p_red!(t, "[error]: {}\n", e.description());
        process::exit(1)
    });
    crates.sort_by(|c1, c2| c2.downloads.cmp(&c1.downloads));

    p_white!(
        t,
        "scrutch: {} crates found with query: \"{}\". Displaying {}.\n\n",
        total,
        args.arg_query,
        crates.len()
    );
    let max_len =
        (&crates).iter().map(|ref cr| cr.name.len()).max().unwrap();
    for cr in &crates {
        show_crate(&mut t, &cr, args.flag_info, max_len);
    }
    t.reset().unwrap();
}

fn show_crate(t: &mut Box<term::StdoutTerminal>, cr: &EncodableCrate, info: bool, max_len: usize) {

    p_green!(t, "{}", cr.name.pad_to_width(max_len));
    p_white!(
        t,
        " = \"{}\"    \t(downloads: {})\n",
        cr.max_version,
        cr.downloads
    );

    if info {
        cr.description.as_ref()
            .map(|description|
                 p_yellow!(t, " -> {}\n", description.clone().trim())
            );
        cr.documentation.as_ref()
            .map(|documentation|
                 p_white!(t, "    docs: {}\n", documentation)
            );
        cr.homepage.as_ref()
            .map(|homepage|
                 p_white!(t, "    home: {}\n", homepage)
            );
        p_white!(t, "\n");
    }
}
