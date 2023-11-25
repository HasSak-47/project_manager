mod daemon;
use std::fs::File;
use std::io::Write;

use crate::error::ProjectResult;

use crate::Manager;
use crate::EDITION;
use crate::utils;


pub trait CliUtil{
    fn run() -> ProjectResult<()>;
}

pub struct PrintPercentajes();
pub struct SelectRandomProject();
pub struct UpdateStatusEdition();

impl CliUtil for PrintPercentajes{
    fn run() -> ProjectResult<()> {
        let manager = Manager::get_config();
        let mut projects = manager.get_unbroken_projects();

        projects.sort_by(|a,b|{
            let ac = a.get_completion();
            let bc = b.get_completion();
            bc.partial_cmp(&ac).unwrap()
        });


        println!("pm version {}", EDITION);
        for p in projects{
            print!("{:20}: {:>7.2}%", p.project.name, p.get_completion() * 100.);
            if p.project.edition != EDITION {
                print!(" --config out date! '{}'--", p.project.edition);
            }
            println!();
        }
        Ok(())
    }
}

impl CliUtil for UpdateStatusEdition{
    fn run() -> ProjectResult<()> {
        use crate::update::v0_1_0;
        let manager = Manager::get_config();
        for st in manager.projects{
            let path = format!("{}/status.toml", st.path);
            let itoml : v0_1_0::prev::ProjectToml = toml::from_str(&utils::read_file(&path)?)?;
            if itoml.project.edition == crate::EDITION{
                continue;
            }
            let otoml : v0_1_0::next::ProjectToml = itoml.into();
            let pth = format!("{}/status.toml", st.path);
            let mut file = File::create(&pth)?;
            file.write_all(toml::to_string(&otoml)?.as_bytes())?;
        }
        Ok(())
    }
}

impl CliUtil for SelectRandomProject{
    fn run() -> ProjectResult<()> {
        use rand::random;
        let manager = Manager::get_config();
        let projects = manager.get_unbroken_projects();
        println!("do: {}", projects[random::<usize>() % projects.len()].project.name);
        Ok(())
    }
}
