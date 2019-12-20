
extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    // recup --in --out
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("JP R. <jp@gmail.com>")
                          .about("Does a pipeline")
                          .arg(Arg::with_name("input")
                               .short("i")
                               .long("in")
                               .value_name("in")
                               .help("This is the input of the pipe")
                               .takes_value(true)
                            )
                          .arg(Arg::with_name("output")
                               .short("o")
                               .long("out")
                               .value_name("out")
                               .help("This is the output of the pipe")
                               .required(true)
                               .takes_value(true)
                            )
                          .get_matches();

    //recup arg after --in
    let param_input = matches.value_of("input").unwrap( );
    
    //recup arg after --out
    let param_output = matches.value_of("output").unwrap( );
                
    
    //exec arg after --in
    let command_input = Command::new(param_input.to_string())
                        .output()
                        .expect("error to exec the arg");
                      
    //exec arg after --out                   
    let command_output = Command::new(param_output.to_string())
                        .arg(String::from_utf8_lossy(&command_input.stdout).to_string())
                        .output()
                        .expect("error to exec the arg");
                      
    println!("{}", String::from_utf8_lossy(&command_output.stdout).to_string());
    

    
}

