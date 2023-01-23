mod utils;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use utils::{args::CliArgs, io::fuzzy_search};

fn main() {
    let mut args = CliArgs::get_args();
    let mut path_string = args.path.unwrap_or(String::new());

    let mut path = Path::new(path_string.as_str());
    let mut tmp_current_pathbuf = PathBuf::new();
    if path_string.is_empty() {
        tmp_current_pathbuf = current_dir().expect("");
        path = tmp_current_pathbuf.as_path();
    }

    let file_name = args.name.expect("[-] Error  Expecting Valid File Name");
    let mut results: Vec<String> = Vec::<String>::new();
    println!("[+] Starting ");
    fuzzy_search(
        &file_name.as_str(),
        &args.is_directory.unwrap_or(false),
        &mut path,
        &mut None,
        &mut args.count.unwrap_or(1),
        &args.verbose.unwrap_or(false),
        &args.backward_search.unwrap_or(false),
        &mut results,
    );
    println!("[+] Results : ");
    for i in results {
        println!("{}", i);
    }
}
