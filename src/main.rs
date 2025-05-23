mod update;
mod config;
mod error;
mod utils;
mod baked;

use config::project::Feature;
use error::*;
#[allow(unused_imports)]
use config::{manager::Manager, project};
use std::env::args;

const EDITION : &str = env!("CARGO_PKG_VERSION");

fn main() -> ProjectResult<()>{
    let manager = Manager::get_config();
    let mut projects = manager.get_unbroken_projects();

    projects.sort_by(|a,b|{
        let ac = a.get_completion();
        let bc = b.get_completion();
        bc.partial_cmp(&ac).unwrap()
    });


    println!("config version {}", EDITION);
    for p in projects{
        print!("{:20}: {:>7.2}%", p.info.name, p.get_completion() * 100.);
        if p.info.edition != EDITION {
            print!(" config out date! '{}'", p.info.edition);
        }
        println!();
    }

    /*
    let pro = project::Project::load_project(".")?;
    println!("DONE");
    print_feat_v(&pro.done);
    println!("TODO");
    print_feat_v(&pro.todo);
    */


    Ok(())
}
