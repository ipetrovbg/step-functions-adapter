use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Pointer, write};
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
pub struct StateMachineDefinition {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "StartAt")]
    pub start_at: String,
    #[serde(rename = "States")]
    pub states: BTreeMap<String, Step>
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
    #[serde(rename = "End")]
    pub end: Option<bool>,
    #[serde(rename = "Next")]
    pub next: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StateMachines {
    pub name: String,
    pub definition: StateMachineDefinition
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Task => {
                writeln!(f, "  \"Type\": \"Task\",")?;
            }
            Type::Pass => {
                writeln!(f, "  \"Type\": \"Pass\",")?;
            }
            Type::Choice => {
                writeln!(f, "  \"Type\": \"Choice\",")?;
            }
            Type::Fail => {
                writeln!(f, "  \"Type\": \"Fail\",")?;
            }
            Type::Succeed => {
                writeln!(f, "  \"Type\": \"Succeed\",")?;
            }
            Type::Wait => {
                writeln!(f, "  \"Type\": \"Wait\",")?;
            }
            Type::Map => {
                writeln!(f, "  \"Type\": \"Map\",")?;
            }
            Type::Parallel => {
                writeln!(f, "  \"Type\": \"Parallel\",")?;
            }
        }
        Ok(())
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        print!("  {}", self.step_type);
        match self.step_type {
            Type::Task => {
                write!(f, "        \"Resource\": \"arn:aws:states:::lambda:invoke\",")?;
            }
            _ => {}
        }

        match self.end {
            None => {
                writeln!(f, "")?;
            }
            Some(end) => {
                if end {
                    write!(f, "        \"End\": true")?;
                }
            }
        }
        match &self.next {
            None => {
                writeln!(f, "")?;
            }
            Some(next) => {
                writeln!(f, "        \"Next\": \"{}\"", next)?;
            }
        }
        Ok(())
    }
}

impl Display for StateMachineDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.comment {
            None => {}
            Some(comment) => {
                writeln!(f, "  \"Comment\": \"{}\",", comment)?;
            }
        }

        writeln!(f, "  \"StartAt\": \"{}\",", self.start_at)?;

        if !self.states.is_empty() {
            writeln!(f, "  \"States\": {{")?;
            for (key, step) in self.states.iter() {
                writeln!(f, "    \"{}\": {{", key)?;
                print!("    {}", step);
                writeln!(f, "    }},")?;
            }
            writeln!(f, "  }}")?;
        }

        Ok(())
    }
}

impl Display for StateMachines {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        print!("{}", self.definition);
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFunctions {
    pub validate: bool,
    #[serde(rename = "stateMachines")]
    pub state_machines: BTreeMap<String, StateMachines>,
}
