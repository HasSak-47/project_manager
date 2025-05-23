use std::env;

mod daemon;
mod configs;
mod tracker;
mod editor;

use configs::project::{self, Project};
use configs::config::{self, Config};

fn main(){
    let config : Config = Config::get_config();//toml::from_str(config_data.as_str()).unwrap();
    let projects = Project::get_projects(&config).unwrap();

    // tracker::main(data, projects);
    let mut args = Vec::<String>::new();
    for arg in env::args() {
        args.push(arg);
    }

    if args.len() >= 2 {
        if args[1] == "--daemon"{
            daemon::main(config, projects); 
        }
        else 
        if args[1] == "--create" {
            editor::make_project(config);
        }
    }
    else{
        tracker::main(config, projects);
    }
}
