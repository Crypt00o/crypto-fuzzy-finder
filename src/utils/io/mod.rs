use std::path::Path;

pub fn fuzzy_search<'fuz>(
    name_of_file: &str,
    is_directory: &bool,
    initial_path: & 'fuz Path,
    except_path: &mut Option<&'fuz Path>,
    results_size: &mut usize,
    debug: &bool,
    backward_search: &bool,
    results: &mut Vec<String>,
) {
    if *debug {
        println!(
            "Searching inside : {} ",
            initial_path.to_str().unwrap_or_default()
        );
    }

    let mut initial_path: &Path = initial_path;

    if *results_size == 0 {
        return;
    }

    match initial_path.read_dir() {
        Ok(readdir_result) => {
            for readdir_result in readdir_result {
                match readdir_result {
                    Ok(dir_entry) => {
                        if except_path.is_some()
                            && dir_entry.path().file_name() == except_path.unwrap().file_name()
                        {
                            continue;
                        } else if dir_entry.path().is_dir() == *is_directory
                            && dir_entry.path().file_name().unwrap_or_default() == name_of_file
                        {
                            results.push(
                                dir_entry
                                    .path()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap_or(String::new()),
                            );

                            *results_size -= 1;
                            if *debug {
                                println!("founded : {}  ", results.last().unwrap());
                            }
                            continue;
                        } else if dir_entry.path().is_dir() {
                            fuzzy_search(
                                name_of_file,
                                is_directory,
                                dir_entry.path().as_path(),
                                &mut None,
                                results_size,
                                debug,
                                &false,
                                results,
                            );
                            continue;
                        } else {
                            continue;
                        }
                    }
                    Err(err) => {
                        if *debug {
                            println!("[-] Error While Reading : {} ", err);
                        }
                        continue;
                    }
                }
            }
        }
        Err(err) => {
            if *debug {
                println!(
                    "[-] Error While Reading Directory {}   : {} ",
                    initial_path.to_str().unwrap_or_default(),
                    err
                );
            }
        }
    };

    if *backward_search {
         *except_path=  Some(initial_path);

        initial_path = match initial_path.parent() {
            Some(value) => value,
            None => return,
        };

        fuzzy_search(
            name_of_file,
            is_directory,
            initial_path,
            except_path,
            results_size,
            debug,
            backward_search,
            results,
        );
    }
}
