mod daemon;
use crate::error::ProjectResult;

use crate::Manager;
use crate::EDITION;


pub trait CliUtil{
    fn run() -> ProjectResult<()>;
}

pub struct PrintPercentajes();
pub struct SelectRandomProject();

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
