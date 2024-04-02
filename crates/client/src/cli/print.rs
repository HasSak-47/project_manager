#![allow(unused_import_braces)]
use std::path::PathBuf;

use crate::SystemHandler;
use super::Arguments;
use libc::STDOUT_FILENO;
use rand::random;
use project_manager_api::{config::manager::Location, format_project, CachedProject, FindCriteria};
use clap::{Subcommand, Args, ValueEnum};

use anyhow::Result;

// this looks like shit
#[derive(Args, Debug, Default, Clone)]
pub struct PrintStruct{
    #[command(subcommand)]
    print: Option<PrintEnum>,
    #[clap(short, long)]
    not_pretty : bool
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
    sort_by: SortBy,
    #[clap(value_enum, long)]
    reversed: bool

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
    let mut path =
        if let Location::Path{path: p} = project.get_location() { p.clone() }
        else { PathBuf::new() };
    // format path to be prettier
    // removes home from path and saves to string with the ~
    let mut processed_path = String::new();
    if let Some(home) = dirs::home_dir(){
        if path.starts_with(&home){
            path = path.strip_prefix(home).unwrap().to_path_buf();
            // removes status.toml
            processed_path.push_str("~/");
            processed_path.push_str(path.to_str().unwrap());
        }
    }
    // prints the project name colored blue bold with pathing to the right an @ and the path
    format!("\x1b[1;34m{name:<width$}\x1b[0m @ {processed_path}", width = padding)
}


impl PrintProjects {
    fn run(&self, mut handler: SystemHandler, args: Arguments, print_args: &PrintStruct) -> Result<()>{
        handler.load_projects();
        let mut projects = handler.get_projects_mut();
        let mut padding = 0usize;
        for p in &projects{
            let l = p.get_name().len();
            if l > padding {padding = l}
        }
        let sort_by = self.sort_by.clone();
        projects.sort_by(|a, b| {
            match sort_by{
                SortBy::Progress => b.get_completion().total_cmp(&a.get_completion()),
                SortBy::Name => a.get_name().cmp(b.get_name()),
                SortBy::LastUsed => b.get_last_updated().cmp(&a.get_last_updated()),
                SortBy::None => a.get_name().cmp(b.get_name()),
            }
        });

        if self.reversed{
            projects.reverse();
        }

        let terminal = unsafe {
            let isatty = libc::isatty(STDOUT_FILENO);
            isatty == 1
        };

        if !print_args.not_pretty && terminal{
            PrintProjects::print_pretty(projects, padding);
        }
        else{
            for p in projects{
                let path = p.get_location().to_string();
                println!("{path}");
            }
        }


        Ok(())
    }

    fn print_pretty(projects: Vec<&mut CachedProject>, padding: usize){
        let mut buffer = Vec::new();
        let mut max = 0usize;
        for p in projects{
            let c = format!("{}", make_pretty(p, padding));
            if c.len() > max {max = c.len()}
            buffer.push(c);
        }
        max += 4;

        let width = unsafe {
            let mut buffer = libc::winsize{ws_col: 0, ws_row: 0, ws_xpixel: 0, ws_ypixel: 0};
            libc::ioctl(0, libc::TIOCGWINSZ, &mut buffer);
            buffer.ws_col
        } as usize / max;


        for (i, p) in buffer.into_iter().enumerate(){
            print!("{:width$}", p, width = max);
            if i % width == width - 1 {println!();}
        }
        println!();

    }
    
}

impl PrintProject{
    fn print(&self, mut handler: SystemHandler){
        let project = handler.find_project_mut(&FindCriteria::name(self.name.clone())).cloned();
        let loader = handler.get_loader();
        match project {
            Ok(mut s) => {
                s.get_completion_mut();
                s.load_project(loader);
                if !self.toml{ println!("{}", format_project(&s)); }
                else{ panic!("I can not print project toml!!"); }
            }
            Err(_) => println!("project not found"),
        }
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
            PE::Project(p) => {
                drop(projects);
                p.print(handler);
            }
            PE::Random => {print_random(projects);}
            PE::Percentajes(p) => {p.print(projects, args)?}
            PE::Broken => {
                drop(projects);
                print_broken_projects(handler)?
            }
            PE::Projects(p) => {
                drop(projects);
                p.run(handler, args, &self)?;
            }
            _ => {},
        }

        Ok(())
    }
}

