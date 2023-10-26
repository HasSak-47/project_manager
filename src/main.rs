mod config;
mod error;
mod utils;
mod baked;

use error::*;
#[allow(unused_imports)]
use config::{manager::Manager, project};

use std::env::args;

fn main() -> ProjectResult<()>{
    let manager = Manager::get_config();
    let mut projects = manager.get_unbroken_projects();

    projects.sort_by(|a,b|{
        let ac = a.get_completion();
        let bc = b.get_completion();
        bc.partial_cmp(&ac).unwrap()
    });

    for p in projects{
        println!("{:20}: {:>7.2}%", p.info.name, p.get_completion() * 100.);
    }


    Ok(())
}
