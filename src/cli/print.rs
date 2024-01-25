use super::RunCmd;
use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::manager::Manager};

// this looks like shit
#[derive(Args, Debug, Clone, Copy)]
pub struct PrintStruct{
    #[command(subcommand)]
    print: Option<PrintEnum>
}

#[derive(Subcommand, Default, Debug, Clone, Copy)]
pub enum PrintEnum{
    Percentaje{
        #[clap(short, long)]
        min: Option<u8>,
        #[clap(short = 'M', long)]
        max: Option<u8>,
    },
    Random,
    #[default]
    None,
}

impl RunCmd for PrintStruct{
    fn run(&self) -> ProjectResult<()> {
        let print = match self.print{
            None => PrintEnum::default(),
            Some(s) => s,
        };

        let manager = Manager::load_data_from(Manager::get_path()?).unwrap();
        let projects = manager.get_unbroken_projects();

        let lines = if let PrintEnum::Percentaje{min, max} = print{
            let min = min.unwrap_or(0) as f32 / 100.;
            let max = max.unwrap_or(100) as f32 / 100.;
            projects.into_iter().filter(|p|{
                let todo = p.get_todo();
                 min < todo && todo  < max
            }).map(|p|{
                let todo = p.get_todo() as f32;
                let done = p.get_done() as f32;
                let total = todo + done;
                format!("{}: {:.2}", p.project.name, (done / total) * 100.) 
            }).collect()
        }else {
            let mut lines = Vec::new();
            for project in projects{
                lines.push(format!("{}", project.project.name));
            }

            lines
        };
        for line in lines{
            println!("{line}");
        }

        Ok(())
    }
}
