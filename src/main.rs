mod model;

use clap::Parser;
use std::fs;

extern crate serde_yaml;

use model::{Serverless, Adapter, Commands};

fn main() {
    let args = Adapter::parse();

    match args.command {
        Commands::Parse { path } => {
           match fs::read_to_string(path.as_path()) {
                Ok(content) => {
                    match serde_yaml::from_str::<Serverless>(&content) {
                        Ok(serverless) => {
                            match serverless.step_functions {
                                None => {
                                    println!("yaml file doesn't contain any step functions definitions!")
                                }
                                Some(step_functions) => {
                                    println!("[");
                                    let mut iteration = 0;
                                    for (_, state_machine) in step_functions.state_machines.iter() {
                                        if (iteration + 1) == step_functions.state_machines.len() {
                                            println!("{}", state_machine);
                                        } else {
                                            println!("{},", state_machine);
                                        }

                                        iteration += 1;
                                    }
                                    print!("]");
                                }
                            }
                        }
                        Err(e) => {
                            print!("Couldn't parse yml file due to: \"");
                            print!("{}\"", e)
                        }
                    }
                }
                Err(_) => {
                    println!("File path wasn't valid!");
                }
            };
        }
    }
}
