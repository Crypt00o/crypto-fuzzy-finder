use std::{env::{Args,args}, process::exit};




pub struct CliArgs{
    
    pub path:Option<String>,
    pub name:Option<String>,
    pub is_directory:Option<bool>,
    pub count:Option<usize>,
    pub backward_search:Option<bool>,
    pub verbose:Option<bool>
}


impl  CliArgs{
    
    pub fn get_args()->Self{
       
       let mut given_args:Args=args();
       let mut arg_counter:usize=1;

       let mut parsed_args=Self{
            path:None,
            name:None,
            count:None,
            backward_search:None,
            is_directory:None,
            verbose:None
          };   
       
        
       while arg_counter<given_args.len() {
         
            match given_args.nth(arg_counter) {
               Some(arg_value)=>{
                    match arg_value.to_lowercase().trim() {
                        "-p" | "--path"=>{
                            match given_args.nth(arg_counter+1) {
                                Some(path_value)=>{
                                
                                   parsed_args.path=Some(path_value);
                                },
                                None=>break
                            };
                            arg_counter+=2; 
                        },
                        "-n" | "--name"=>{

                            match given_args.nth(arg_counter+1) {
                                Some(name_value)=>{
                                     parsed_args.name=Some(name_value); 
                                },
                                None=>break
                            };
                            arg_counter+=2; 

                        },
                        "-c" | "--count"=>{
                            match given_args.nth(arg_counter+1) {
                                Some(count_value)=>{
                                     parsed_args.name= Some(count_value.parse().expect("[-] Error Count Should Be Positive Number"));
                                },
                                None=>break
                            };
                            arg_counter+=2; 
                        },
                        "-b" | "--backward"=>{
                           parsed_args.backward_search=Some(true);
                            arg_counter+=1;

                        },
                        "-f" | "--file"=>{
                            
                            parsed_args.is_directory=Some(false);
                            arg_counter+=1;

                        },
                        "-d" | "--dir"=>{

                            parsed_args.is_directory=Some(true);
                            arg_counter+=1;

                        },
                        "-v" | "--verbose" =>{

                        },
                        "-h" | "--help"=>{

                            Self::usage();
                        
                        }
                        _=>{
                            println!("[-] Unknown Option  : {}",arg_value);
                            exit(1)
                        }
                    }
               },
               None=>break
            }

        }
        if parsed_args.name.is_none(){
            Self::usage()
        }
       return parsed_args;
    }

    fn usage(){
        println!("{}",r"
                                    
             [+] Crypt00o Fuzzy Finder [+]
            
            finding Files is Fast & Easy With crypto-fuzzy ;)

                https://github.com/Crypt00o/
[+] Usage:
    
    --path      or -p => setting initial path to search in

    --name      or -n => setting name of file to search 

    --count     or -c => setting count of files to search   

    --dir       or -d => setting filetype to Directory to get all Directories which matching your file name

    --file      or -f => setting filetype to File to get all Files Which matching your file name

    --backward  or -b => setting flag backward on which mean if excepted file or count of file didn,t match initial path result,it will search in the parents dir untill get it

    --verbose   or -v => showing debugging info while searching

        ");
        exit(1);

    }

}
