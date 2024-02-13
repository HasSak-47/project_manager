#![allow(unused_import_braces)]
use crate::SystemHandler;
use super::Arguments;
use rand::random;
use project_manager_api::CachedProject;
use clap::{Subcommand, Args, ValueEnum};

use anyhow::Result;

// this looks like shit
#[derive(Args, Debug, Default, Clone)]
pub struct PrintStruct{
    #[command(subcommand)]
    print: Option<PrintEnum>,
}

#[derive(Args, Debug, Default, Clone)]
struct PrintPercentaje{
    #[clap(short, long, default_value = "0")]
    min: u8,
    #[clap(short = 'M', long, default_value = "100")]
    max: u8,
    #[clap(short, long)]
    unsorted: bool,
}

#[derive(Args, Debug, Default, Clone)]
struct PrintProject{
    name: String,
    #[clap(short, long)]
    toml: bool,
    // probably shouldn't be used like this
}

#[derive(Args, Debug, Default, Clone)]
struct PrintProjects{
    #[clap(value_enum, long, default_value = "progress")]
    sort_by: SortBy
}

#[derive(ValueEnum, Default, Debug, Clone)]
enum SortBy{
    #[default]
    Progress,
    Name,
    LastUsed,
    #[clap(skip)]
    None,
}


#[derive(Subcommand, Default, Debug, Clone)]
enum PrintEnum{
    Percentajes(PrintPercentaje),
    Random,
    Project(PrintProject),
    Projects(PrintProjects),
    #[default]
    #[clap(skip)]
    None,
}

fn print_projects(mut projects: Vec<&mut CachedProject>, _data: PrintProjects) -> Result<()>{
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.get_name().len();
        if l > max_len {max_len = l}
    }

    for project in &mut projects{
        if project.cache_completion().is_err() {
            println!("project: {project:#?} could not be loaded");
        }
    }
    
    if let SortBy::None = _data.sort_by{ }
    else{
        projects.sort_by(match _data.sort_by {
            _ => |a: &&mut CachedProject, b: &&mut CachedProject|{ b.get_completion().total_cmp(&a.get_completion()) },
        })
    };

    for p in projects{
        println!("{:2$}{:.2}%", p.get_name(), p.get_completion() * 100., max_len + 4);
    }
    Ok(())
}

fn print_project(projects: Vec<&mut CachedProject>, data: PrintProject){
    match projects.iter().find(|p| *p.get_name() == data.name){
        Some(s) => {
            if !data.toml{
                println!("{}: {s:?}", data.name);
            }
            else{
                panic!("not implemented!");
            }
        }
        None => println!("project not found"),
    }

}

fn print(projects: Vec<&mut CachedProject>){
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.get_name().len();
        if l > max_len {max_len = l}
    }
    for p in projects{
        println!("{:1$}", p.get_name(), max_len);
    }
}

fn print_random(projects: Vec<&mut CachedProject>){
    let i = random::<usize>() % projects.len();
    println!("{}", projects[i].get_name());
}

impl PrintStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()> {
        handler.load_projects();

        let projects = handler.get_projects_mut();
        let option = if self.print.is_none(){
            PrintEnum::default()
        }
        else{
            self.print.clone().unwrap()
        };
        use PrintEnum as PE;
        match option{
            PE::Project(p) => {print_project(projects, p);}
            PE::Projects(p) => {print_projects(projects, p)?;}
            PE::Random => {print_random(projects);}
            _ => {print(projects);},
        }

        Ok(())
    }
}

