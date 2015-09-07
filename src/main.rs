#![allow(unused_must_use)]
extern crate curl;
extern crate docopt;
extern crate rustc_serialize;
extern crate term;
extern crate url;

use std::str;

use curl::http;
use docopt::Docopt;
use rustc_serialize::json;
use url::percent_encoding::{QUERY_ENCODE_SET, utf8_percent_encode};

#[macro_use]
mod macros;

const USAGE: &'static str = "
Scrutch - Crates Search

Usage:
  scrutch [--info] <query>
  scrutch (-h | --help)
  scrutch --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --info        Show complete details of the crates.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_info: bool,
    arg_query: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Meta {
    total: i32,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Response {
    crates: Vec<EncodableCrate>,
    meta: Meta,
}

// structs from crates.io backend

#[derive(Debug, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct CrateLinks {
    version_downloads: String,
    versions: Option<String>,
    owners: Option<String>,
    reverse_dependencies: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                          .and_then(|d| d.decode())
                          .unwrap_or_else(|e| e.exit());

    let mut t = term::stdout().unwrap();

    // TODO: think about implementing page and per_page via an option, maybe
    let url = format!(
    "https://crates.io/api/v1/crates?q={}", //&page=1&per_page=10", 
    utf8_percent_encode(&args.arg_query, QUERY_ENCODE_SET)
  );
    let res = match http::handle().get(url).exec() {
        Ok(res) => res,
        Err(e) => {
            p_red!(t, "[error]: unable to retrieve data - {}\n", e);
            return;
        }
    };
    let body = str::from_utf8(res.get_body()).unwrap();
    let mut data: Response = match json::decode(&body) {
        Ok(body) => body,
        Err(e) => {
            p_red!(t, "[error]: unable to parse json - {}\n", e);
            return;
        }
    };

    // TODO: Add decoding of updated_at and allow to use it for sorting
    let mut crates = &mut data.crates[..];
    crates.sort_by(|c1, c2| {c2.downloads.cmp(&c1.downloads)});

    p_white!(t, "scrutch: {} crates found with query: \"{}\"\n\n", 
             crates.len(), args.arg_query);
    for cr in crates {
        show_crate(&mut t, cr, args.flag_info);
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

    p_white!(t, " = \"{}\"\t(downloads: {})\n", cr.max_version, cr.downloads);

    if info {
        if_some!(&cr.description, 
                 p_yellow!(t, " -> {}\n", &cr.description.clone().unwrap().trim()));
        if_some!(&cr.documentation, 
                 p_white!(t, "    docs: {}\n", &cr.documentation.clone().unwrap()));
        if_some!(&cr.homepage, 
                 p_white!(t, "    home: {}\n", &cr.homepage.clone().unwrap()));
        p_white!(t, "\n");
    }
}
