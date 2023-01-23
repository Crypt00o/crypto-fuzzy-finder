use std::{env::args, process::exit};

pub struct CliArgs {
    pub path: Option<String>,
    pub name: Option<String>,
    pub is_directory: Option<bool>,
    pub count: Option<usize>,
    pub backward_search: Option<bool>,
    pub verbose: Option<bool>,
}

impl CliArgs {
    pub fn get_args() -> Self {
        let mut arg_counter: usize = 1;

        let mut parsed_args = Self {
            path: None,
            name: None,
            count: None,
            backward_search: None,
            is_directory: None,
            verbose: None,
        };
        let mut given_args: Vec<String> = args().collect::<Vec<String>>();

        while arg_counter < given_args.len() {
            match given_args[arg_counter].clone().as_str() {
                "-p" | "--path" => {
                    arg_counter += 1;
                    parsed_args.path = Some(given_args[arg_counter].clone());
                }
                "-n" | "--name" => {
                    arg_counter += 1;
                    parsed_args.name = Some(given_args[arg_counter].clone());
                }
                "-c" | "--count" => {
                    arg_counter += 1;
                    parsed_args.count = Some(
                        given_args[arg_counter]
                            .parse()
                            .expect("[-] Error Count Should Be Positive Number"),
                    );
                }
                "-f" | "--file" => {
                    parsed_args.is_directory = Some(false);
                }
                "-d" | "--dir" => {
                    parsed_args.is_directory = Some(true);
                }
                "-v" | "--verbose" => {
                    parsed_args.verbose = Some(true);
                }
                "-b" | "--backward" => {
                    parsed_args.backward_search = Some(true);
                }

                "-h" | "--help" => {
                    Self::usage();
                }

                _ => {
                    println!("[-] Unknown Option  : {}", given_args[arg_counter]);
                    exit(1);
                }
            }
            arg_counter += 1;
        }

        if parsed_args.name.is_none() {
            Self::usage()
        }
        return parsed_args;
    }

    fn usage() {
        println!(
            "{}",
            r"
                                    
             [+] Crypt00o Fuzzy Finder [+]
            
            finding Files is Fast & Easy With crypto-fuzzy ;)

                https://github.com/Crypt00o/crypto-fuzzy-finder
[+] Usage:
    
    --path      or -p => setting initial path to search in

    --name      or -n => setting name of file to search 

    --count     or -c => setting count of files to search   

    --dir       or -d => setting filetype to Directory to get all Directories which matching your file name

    --file      or -f => setting filetype to File to get all Files Which matching your file name

    --backward  or -b => setting flag backward on which mean if excepted file or count of file didn,t match initial path result,it will search in the parents dir untill get it

    --verbose   or -v => showing debugging info while searching

        "
        );
        exit(1);
    }
}
