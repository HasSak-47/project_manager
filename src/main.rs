mod config;
mod error;
mod utils;
mod baked;

use config::project::Feature;
use error::*;
#[allow(unused_imports)]
use config::{manager::Manager, project};
use std::env::args;

fn __print_feat_v(v: &Vec<Feature>, level : i32){
    for f in v{
        println!("{f:?}");
        if f.todo.len() != 0{ println!("todo:") }
        __print_feat_v(&f.todo, level + 1);
        if f.done.len() != 0{ println!("done:") }
        __print_feat_v(&f.done, level + 1);
    }

}

fn print_feat_v(v: &Vec<Feature>){
    __print_feat_v(v, 0)
}

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

    /*
    let pro = project::Project::load_project(".")?;
    println!("DONE");
    print_feat_v(&pro.done);
    println!("TODO");
    print_feat_v(&pro.todo);
    */


    Ok(())
}
