#![allow(unused_import_braces)]
use std::path::PathBuf;

use crate::SystemHandler;
use super::Arguments;
use rand::random;
use project_manager_api::{config::manager::Location, CachedProject};
use clap::{Subcommand, Args, ValueEnum};

use anyhow::Result;

// this looks like shit
#[derive(Args, Debug, Default, Clone)]
pub struct PrintStruct{
    #[command(subcommand)]
    print: Option<PrintEnum>,
    #[clap(short, long)]
    pretty : bool
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

#[derive(Args, Default, Debug, Clone)]
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
    #[allow(unused)]
    #[clap(skip)]
    None,
}


#[derive(Subcommand, Default, Debug, Clone)]
enum PrintEnum{
    Percentajes(PrintPercentaje),
    Random,
    Project(PrintProject),
    Projects(PrintProjects),
    Broken,
    #[default]
    #[clap(skip)]
    None,
}

fn print_broken_projects(mut handler: SystemHandler) -> Result<()>{
    handler.load_projects();
    let projects = handler.get_projects();
    for project in projects{
        if project.broken().is_some_and(|f| f == true) {
            println!("project {} status is broken", project.get_name());
            println!("{:?}", project)
        }
    }
    Ok(())
}

fn make_pretty(project: &CachedProject, padding: usize) -> String {
    let name = project.get_name();
    let path =
        if let Location::Path{path: p} = project.get_location() { p.clone() }
        else { PathBuf::new() };
    format!("\x1b[1;34m{name:1$}\x1b[0m @ {}", path.display(), padding)
}

fn print_projects(projects: Vec<&mut CachedProject>, _data: PrintProjects) -> Result<()>{
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.get_name().len();
        if l > max_len {max_len = l}
    }


    for p in projects{
        println!("{}", make_pretty(p, max_len + 4));
    }
    Ok(())
}

impl PrintProject{
    fn print(&self, projects: Vec<&mut CachedProject>){
        match projects.iter().find(|p| *p.get_name() == self.name){
            Some(s) => {
                if !self.toml{ println!("{}: {s:?}", self.name); }
                else{ panic!("I can not print project toml!!"); }
            }
            None => println!("project not found"),
        }
    
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


impl PrintPercentaje{
    fn print(&self, mut projects: Vec<&mut CachedProject>, args : Arguments) -> Result<()>{
        let mut max_len = 0usize;
        for p in &projects{
            let l = p.get_name().len();
            if l > max_len {max_len = l}
        }

        for project in &mut projects{
            if project.cache_completion().is_err() && args.verbose && args.debug{
                println!("project: {project:#?} could not be loaded");
            }
        }

        let mut filtered : Vec<_> = projects.iter().filter(|p| {
            self.min as f64 <= p.get_completion() * 100. &&
                p.get_completion() * 100. <= self.max as f64}).collect();
        
        filtered.sort_by(|a, b| b.get_completion().total_cmp(&a.get_completion()) );

        for p in filtered{
            println!("{:2$} {:>7.2}%", p.get_name(), 100. * p.get_completion(), max_len + 4);
        }
        Ok(())
    }
}

impl PrintStruct{
    pub fn run(self, args: Arguments, mut handler: SystemHandler) -> Result<()> {
        handler.load_projects();

        let projects = handler.get_projects_mut();
        let option = if self.print.is_none(){
            PrintEnum::Projects(PrintProjects::default() )
        }
        else{
            self.print.clone().unwrap()
        };
        use PrintEnum as PE;
        match option{
            PE::Project(p) => {p.print(projects);}
            PE::Random => {print_random(projects);}
            PE::Percentajes(p) => {p.print(projects, args)?}
            PE::Broken => {
                drop(projects);
                print_broken_projects(handler)?
            }
            PE::Projects(p) => {print_projects(projects, p)?;}
            _ => {},
        }

        Ok(())
    }
}

