#![allow(unused_import_braces)]
use std::path::PathBuf;

use libc::STDOUT_FILENO;
use project_manager_api::{manager::Manager, project::ProjectStatus, CachedProject, FindCriteria, Handler, Location};
use super::Arguments;
use rand::random;
use clap::{Subcommand, Args, ValueEnum};

use anyhow::{anyhow, Result};

// this looks like shit
#[derive(Args, Debug, Default, Clone)]
pub struct PrintStruct{
    #[command(subcommand)]
    print: Option<PrintEnum>,
    #[clap(short, long)]
    not_pretty : bool,
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
    reversed: bool,

    #[clap(short, long)]
    path: bool,
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

// fn print_broken_projects(mut handler: Manager) -> Result<()>{
//     handler.load_projects();
//     let projects = handler.get_projects();
//     for project in projects{
//         if project.broken().is_some_and(|f| f == true) {
//             println!("project {} status is broken", project.get_name());
//             println!("{:?}", project)
//         }
//     }
//     Ok(())
// }
// 
fn make_pretty(project: &CachedProject, padding: usize) -> String {
    let name = project.get_name();
    let mut path =
        if let Location::Path(p) = project.get_location() { p.clone() }
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
    fn run(&self, mut handler: Handler, _args: Arguments, print_args: &PrintStruct) -> Result<()>{
        if _args.debug{
            println!("loading projects...");
        }
        handler.load_projects()?;
        _args.debug(&
            format!("Handler: {handler:?}")
        );
        let mut projects = handler.get_cached_projects();
        if _args.debug{
            println!("projects: {:#?}", projects);
        }
        let mut padding = 0usize;
        for (_, p) in &handler.projects{
            let l = p.project.info.name.len();
            if l > padding {padding = l}
        }
        // let sort_by = self.sort_by.clone();

        if self.reversed{
            projects.reverse();
        }

        let terminal = unsafe { libc::isatty(STDOUT_FILENO) == 1 };
        if !print_args.not_pretty && terminal{
            if !self.path{
                PrintProjects::print_pretty(&projects, padding);
            }
            else{
                for p in projects{
                    println!("{}", p.get_name());
                }
            }
        }
        else{
            if !self.path{
                for p in projects{
                    let path = p.get_location().to_string();
                    let name = p.get_name();
                    println!("{name},{path}");
                }
            }
            else{
                for p in projects{
                    let name = p.get_name();
                    println!("{name}");
                }
            }
        }

        Ok(())
    }

    fn print_pretty(projects: &Vec<CachedProject>, padding: usize){
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
// 
// impl PrintProject{
//     fn print(&self, mut handler: Handler){
//         let project = handler.find_project_mut(&FindCriteria::Name(self.name.clone()));
//         // match project {
//         //     Some(mut s) => {
//         //         s.get_completion_mut();
//         //         s.load_project(loader);
//         //         if !self.toml{ println!("{}", format_project(&s)); }
//         //         else{ panic!("I can not print project toml!!"); }
//         //     }
//         //     None => println!("project not found"),
//         // }
//     }
// }
// 
// fn print_random(projects: Vec<&mut ProjectStatus>){
//     let i = random::<usize>() % projects.len();
//     println!("{}", projects[i].get_name());
// }

// impl PrintPercentaje{
//     fn print(&self, mut handler: Handler, args : Arguments) -> Result<()>{
//         let mut max_len = 0usize;
//         let projects = handler.get_cached_projects();
//         for p in &projects{
//             let l = p.get_name().len();
//             if l > max_len {max_len = l}
//         }
// 
//         for project in &mut projects{
//             if project.cache_completion().is_err() && args.verbose && args.debug{
//                 println!("project: {project:#?} could not be loaded");
//             }
//         }
// 
//         let mut filtered : Vec<_> = projects.iter().filter(|p| {
//             self.min as f64 <= p.get_completion() * 100. &&
//                 p.get_completion() * 100. <= self.max as f64}).collect();
//         
//         filtered.sort_by(|a, b| b.get_completion().total_cmp(&a.get_completion()) );
// 
//         for p in filtered{
//             println!("{:2$} {:>7.2}%", p.get_name(), 100. * p.get_completion(), max_len + 4);
//         }
//         Ok(())
//     }
// }

impl PrintStruct{
    pub fn run(self, args: Arguments, handler: Handler) -> Result<()> {
        let option = if self.print.is_none(){
            if args.debug{
                println!("no subcommand given defaulting to projects");
            }
            PrintEnum::Projects(PrintProjects::default() )
        }
        else{
            self.print.clone().unwrap()
        };
        use PrintEnum as PE;
        match option{
            PE::Projects(p) => p.run(handler, args, &self),
            _ => {Ok(())},
        }
    }
}

