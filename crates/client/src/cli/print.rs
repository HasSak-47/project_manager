#![allow(unused_import_braces)]
use crate::{SystemLoader, SystemHandler};
use super::{Params, Arguments};
use clap::{Subcommand, Args};
use rand::random;
use project_manager_api::{
    error::{ProjectResult},
    config::{
        manager::{Manager, ProjectData},
        project::Project
    }, ProjectLoader, CachedProject,
};

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
    #[clap(subcommand)]
    sort_by: SortBy
}

#[derive(Subcommand, Default, Debug, Clone)]
enum SortBy{
    #[default]
    #[clap(skip)]
    None,

    Progress,
    Name,
    LastUsed,
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

fn print_projects(mut projects: Vec<&mut CachedProject>, _data: PrintProjects){
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.get_name().len();
        if l > max_len {max_len = l}
    }
    
    if let SortBy::None = _data.sort_by{ }
    else{
        projects.sort_by(match _data.sort_by {
            SortBy::Progress => |a: &&mut CachedProject, b: &&mut CachedProject| b.get_completion_mut().total_cmp(&a.get_completion_mut()),
                           _ => |a: &&mut CachedProject, b: &&mut CachedProject| b.get_name().cmp(&a.get_name()),
        })
    };

    for p in projects{
        println!("{:1$}", p.get_name(), max_len + 4);
    }
}

fn print_percentaje(mut projects: Vec<&mut CachedProject>, data: PrintPercentaje){
    if !data.unsorted{
        projects.sort_by(|a, b| b.get_completion_mut().total_cmp(&a.get_completion_mut()));
    }
    let projects : Vec<_> = projects
        .iter()
        .filter(|p| {
            let c = (p.get_completion_mut() * 100.) as u8;
            data.min <= c && c <= data.max
        }).collect();
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.get_name().len();
        if l > max_len {max_len = l}
    }

    for p in projects{
        println!("{:2$}{:>7.2}%", p.get_name(), p.get_completion_mut() * 100., max_len + 4, );
    }
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
        println!("{:1$}{}", p.get_name(), max_len);
    }
}

fn print_random(projects: Vec<&mut CachedProject>){
    let i = random::<usize>() % projects.len();
    println!("{}", projects[i].get_name());
}

impl PrintStruct{
    pub fn run(&self, args: Arguments, mut handler: SystemHandler) -> ProjectResult<()> {
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
            PE::Percentajes(p) => {print_percentaje(projects, p);},
            PE::Project(p) => {print_project(projects, p);}
            PE::Projects(p) => {print_projects(projects, p);}
            PE::Random => {print_random(projects);}
            _ => {print(projects);},
        }

        Ok(())
    }
}

