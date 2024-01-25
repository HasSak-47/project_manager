use super::{RunCmd, Params};
use clap::{Subcommand, Parser, Args};
use rand::random;
use crate::{error::ProjectResult, config::{manager::Manager, project::Project}};

// this looks like shit
#[derive(Args, Debug, Default, Clone)]
pub struct PrintStruct{
    #[command(subcommand)]
    print: Option<PrintEnum>
}

#[derive(Subcommand, Default, Debug, Clone)]
pub enum PrintEnum{
    Percentaje{
        #[clap(short, long)]
        min: Option<u8>,
        #[clap(short = 'M', long)]
        max: Option<u8>,
    },
    Random,
    Project {
        name: String,
        #[clap(short, long)]
        toml: bool
    },
    #[default]
    None,
}

fn print_percentaje(projects: Vec<Project>, min: u8, max: u8){
    for p in projects{
        println!("{} : {}", p.project.name, p.get_completion());
    }
}

fn print_project(projects: Vec<Project>, name: String, toml: bool){
    for p in projects{
        if p.project.name == name {
            if !toml{
                println!("{name} : {p:?}");
            }
            else{
                println!("{name} : {}", toml::to_string_pretty(&p).unwrap());
            }
            return;
        }
    }


}

fn print(projects: Vec<Project>){
    for p in projects{
        println!("{}", p.project.name);
    }
}

fn print_random(projects: Vec<Project>){
    let i = random::<usize>() % projects.len();
    println!("{}", projects[i].project.name);
}

impl RunCmd for PrintStruct{
    fn run(&self, _: Params) -> ProjectResult<()> {
        let manager = Manager::load_data_from(Manager::get_path()?).unwrap();
        let projects = manager.get_unbroken_projects();
        drop(manager);

        let option = if self.print.is_none(){
            PrintEnum::default()
        }
        else{
            self.print.clone().unwrap()
        };
        use PrintEnum as PE;
        match option{
            PE::Percentaje { min, max } => {print_percentaje(projects, min.unwrap_or(0), max.unwrap_or(0));},
            PE::Project { name, toml } => {print_project(projects, name, toml);}
            PE::Random => {print_random(projects);}
            _ => {print(projects);},
        }

        Ok(())
    }
}

