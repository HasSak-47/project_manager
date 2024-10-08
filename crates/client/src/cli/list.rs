use std::path::PathBuf;

use libc::STDOUT_FILENO;
use project_manager_api::{project::ProjectInfo, Handler, Location};
use super::Arguments;
use clap::{Subcommand, Args, ValueEnum};

use anyhow::Result;

// this looks like shit
#[derive(Args, Debug, Default, Clone)]
pub struct ListStruct{
    #[command(subcommand)]
    print: Option<ListEnum>,
    #[clap(short, long)]
    not_pretty : bool,
}

#[derive(Args, Debug, Default, Clone)]
struct ListPercentaje{
    #[clap(short, long, default_value = "0")]
    min: u8,
    #[clap(short = 'M', long, default_value = "100")]
    max: u8,
    #[clap(short, long)]
    unsorted: bool,
}

#[derive(Args, Debug, Default, Clone)]
struct ListProject{
    name: String,
    #[clap(short, long)]
    toml: bool,
    // probably shouldn't be used like this
}

#[derive(Args, Default, Debug, Clone)]
struct ListProjects{
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
enum ListEnum{
    Percentajes(ListPercentaje),
    Random,
    Project(ListProject),
    Projects(ListProjects),
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
fn make_pretty(project: &ProjectInfo, padding: usize) -> String {
    let name = &project.name;
    let mut path =
        if let Location::Path(p) = &project.location { p.clone() }
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

fn print_pretty(projects: &Vec<ProjectInfo>, padding: usize){
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

impl ListProjects {
    fn run(&self, handler: Handler, _args: Arguments, print_args: &ListStruct) -> Result<()>{
        let mut projects = handler.get_projects_info();
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
                print_pretty(&projects, padding);
            }
            else{
                for p in projects{
                    println!("{}", p.location.to_string());
                }
            }
        }
        else{
            if !self.path{
                for p in projects{
                    let path = p.location.to_string();
                    let name = p.name;
                    println!("{name},{path}");
                }
            }
            else{
                for p in projects{
                    let name = p.name;
                    println!("{name}");
                }
            }
        }

        Ok(())
    }
    
}

impl ListProject{
    fn run(&self, mut handler: Handler, _args: Arguments, _ps : &ListStruct) -> Result<()>{
        handler.load_projects()?;
        match handler.get_project(&self.name){
            Ok(project) => {
                println!("Name: {}\nPath: {}", project.info.name, project.info.location.to_string())
            },
            Err(_) => {
                println!("project \"{}\" not found\n", self.name);
            }
        }
        Ok(())

    }
}

impl ListPercentaje{
    fn run(&self, mut handler: Handler, _args: Arguments, _ps : &ListStruct) -> Result<()>{
        handler.load_projects()?;
        let projects = handler.get_cached_projects();
        
        for project in projects{
            let completion = project.get_completion() * 100.;
            if self.min as f64 <= completion && completion <= self.max as f64{
                println!("{:2} {:>7.2}%", project.get_name(), completion);
            }
        }

        Ok(())
    }
}

impl ListStruct{
    pub fn run(self, args: Arguments, handler: Handler) -> Result<()> {
        let option = if self.print.is_none(){
            if args.debug{
                println!("no subcommand given defaulting to projects");
            }
            ListEnum::Projects( ListProjects::default() )
        }
        else{
            self.print.clone().unwrap()
        };
        use ListEnum as PE;
        match option{
            PE::Projects(p) => p.run(handler, args, &self),
            PE::Percentajes(p) => p.run(handler, args, &self),
            PE::Project(p) => p.run(handler, args, &self),
            _ => {Ok(())},
        }
    }
}

