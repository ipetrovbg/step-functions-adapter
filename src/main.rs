mod model;

use clap::Parser;
use std::fs;

extern crate serde_yaml;

use model::{Adapter, Commands, Serverless};

fn main() {
    let args = Adapter::parse();

    match args.command {
        Commands::Parse { path } => {
            match fs::read_to_string(path.as_path()) {
                Ok(content) => match serde_yaml::from_str::<Serverless>(&content) {
                    Ok(serverless) => match serverless.step_functions {
                        None => {
                            eprintln!("yaml file doesn't contain any step functions definitions!")
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
                    },
                    Err(e) => {
                        eprint!("Couldn't parse yml file due to: \"");
                        eprint!("{}\"", e)
                    }
                },
                Err(_) => {
                    eprintln!("File path wasn't valid!");
                }
            };
        }
    }
}
