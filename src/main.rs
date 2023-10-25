mod config;
mod error;
mod utils;

use error::*;
#[allow(unused_imports)]
use config::{manager, project};

fn main() -> ProjectResult<()>{
    let manager = manager::get_config();
    for project in manager.projects{
        let p = project::load_project(project.path)?;
        let p_done = p.get_done();
        let p_todo = p.get_todo();

        let p_total = p_todo + p_done;
        let p_done_p = 100. * if p_total != 0. {p_done / p_total} else {0.};

        println!("{:30} {p_done_p:04.2}", p.info.name);
    }




    Ok(())
}
