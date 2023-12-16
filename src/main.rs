#![allow(unused_imports)]

mod cli;
mod update;
mod config;
mod error;
mod utils;
mod baked;

use config::project::{Feature, Project, ProjectToml};
use error::*;
use config::{manager::Manager, project};
use std::env::args;

#[allow(dead_code)]
const EDITION : &str = "0.1.0";

fn main() -> ProjectResult<()>{
    /*
    let mut args : Vec<String> = args().into_iter().collect();
    args.remove(0);
    let opt = if args.len() == 0 {
        String::from("list")
    } else { args[0].clone() };
    if opt == "update"{
        UpdateStatusEdition::run()?;
    }
    else
    if opt == "random"{
        SelectRandomProject::run()?;
    }
    else{
        PrintPercentajes::run()?;
    }
    */
    Ok(())
}
