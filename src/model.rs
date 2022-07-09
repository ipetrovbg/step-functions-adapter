use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "adapter")]
#[clap(about = "Step Functions Adapter CLI", long_about = "CLI adapter for AWS Step Functions and Serverless framework written in Rust")]
pub struct Adapter {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Parse {
        #[clap(required = true, value_parser)]
        path: PathBuf,
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Serverless {
    #[serde(rename = "stepFunctions")]
    pub step_functions: Option<StepFunctions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StateMachineDefinition {
    #[serde(rename = "Comment")]
    comment: Option<String>,
    #[serde(rename = "StartAt")]
    start_at: String,
    #[serde(rename = "States")]
    states: BTreeMap<String, Step>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Task,
    Pass,
    Choice,
    Fail,
    Succeed,
    Wait,
    Map,
    Parallel,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    #[serde(rename = "Type")]
    pub step_type: Type,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StateMachines {
    name: String,
    definition: StateMachineDefinition
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFunctions {
    validate: bool,
    #[serde(rename = "stateMachines")]
    state_machines: BTreeMap<String, StateMachines>,
}
