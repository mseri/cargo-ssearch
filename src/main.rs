extern crate curl;
extern crate docopt;
extern crate rustc_serialize;

use std::str;

use curl::http;
use docopt::Docopt;
use rustc_serialize::json;

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
    arg_query: String
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
  keywords: Vec<String>,
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

  // TODO: think about implementing page and per_page via an option, maybe
  let url = format!(
    "https://crates.io/api/v1/crates?q={}&page=1&per_page=10", 
    args.arg_query
  );
  let res = match http::handle().get(url).exec() {
      Ok(res) => res,
      Err(e) => { println!("{}", e); return; }
  };
  let body = str::from_utf8(res.get_body()).unwrap();
  let mut data:Response = json::decode(&body).unwrap();

  let mut crates = &mut data.crates[..];
  crates.sort_by(|c1, c2| {c2.downloads.cmp(&c1.downloads)});

  println!("scrutch: {} crates found with query: {}\n", crates.len(), args.arg_query);
  for cr in crates {
    show_crate(cr, args.flag_info);
  }
}

fn show_crate(cr: &EncodableCrate, info: bool) {
  println!("{} (dwl: {})", cr.id, cr.downloads);
  if info {
    print_if_present("", "-> ", &cr.description);
    
    print_if_present("   ", "docs: ", &cr.documentation);
    print_if_present("   ", "home: ", &cr.homepage);
    println!("");
  }
}

fn print_if_present(prefix: &str, label: &str, data: &Option<String>) {
  match data {
    &Some(ref info) => println!("{}{}{}", prefix, label, info),
    &None => return
  }
}