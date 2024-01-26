use super::{RunCmd, Params};
use clap::{Subcommand, Args};
use rand::random;
use crate::{error::ProjectResult, config::{manager::Manager, project::Project}};

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
    toml: bool
}

#[derive(Args, Debug, Default, Clone)]
struct PrintProjects{ }

#[derive(Subcommand, Default, Debug, Clone)]
enum PrintEnum{
    Percentajes(PrintPercentaje),
    Random,
    Project(PrintProject),
    Projects(PrintProjects),
    #[default]
    None,
}

fn print_projects(manager: Manager, projects: Vec<Project>, _data: PrintProjects){
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.project.name.len();
        if l > max_len {max_len = l}
    }

    for p in manager.projects{
        println!("{:2$} {}", p.name, p.path.display(), max_len + 4);
    }
}

fn print_percentaje(mut projects: Vec<Project>, data: PrintPercentaje){
    if !data.unsorted{
        projects.sort_by(|a, b| b.get_completion().total_cmp(&a.get_completion()));
    }
    let projects : Vec<_> = projects
        .iter()
        .filter(|p| {
            let c = (p.get_completion() * 100.) as u8;
            data.min <= c && c <= data.max
        }).collect();
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.project.name.len();
        if l > max_len {max_len = l}
    }

    for p in projects{
        println!("{:2$}{:>7.2}%", p.project.name, p.get_completion() * 100., max_len + 4, );
    }
}

fn print_project(projects: Vec<Project>, data: PrintProject){
    for p in projects{
        if p.project.name == data.name {
            if !data.toml{
                println!("{} : {p:?}", data.name);
            }
            else{
                println!("{} : {}", data.name, toml::to_string_pretty(&p).unwrap());
            }
            return;
        }
    }

    println!("project not found");
}

fn print(projects: Vec<Project>){
    let mut max_len = 0usize;
    for p in &projects{
        let l = p.project.name.len();
        if l > max_len {max_len = l}
    }
    for p in projects{
        println!("{:1$}{}", p.project.name, max_len);
    }
}

fn print_random(projects: Vec<Project>){
    let i = random::<usize>() % projects.len();
    println!("{}", projects[i].project.name);
}

impl RunCmd for PrintStruct{
    fn run(&self, params: Params) -> ProjectResult<()> {
        let manager = Manager::load_data_from(&params.manager_path)?;
        let projects = manager.get_unbroken_projects();
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
            PE::Projects(p) => {print_projects(manager, projects, p);}
            PE::Random => {print_random(projects);}
            _ => {print(projects);},
        }

        Ok(())
    }
}

