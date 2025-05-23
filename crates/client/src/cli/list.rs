

use project_manager_api::Database;

use super::Arguments;
use clap::{Subcommand, Args, ValueEnum};
use ly::log::prelude::*;

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

impl ListProjects{
    fn run(self, mut database: Database) -> Result<()>{

        let entries : Vec<_> = database.get_all_projects().into_iter().map(|p| (p.name().clone(), p.location().clone(), p.get_completion()) ).collect();
        let _ = log!("entry count: {}", entries.len());
        for (n, p, c) in entries{
            let p = match p{
                project_manager_api::Location::Path(p) => p.to_str().unwrap().to_string(),
                _ => "Not here lmao".to_string(),
            };
            println!("{n} @ {p}: {:5.2}%", c * 100.);
        }

        Ok(())
    }
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

impl ListStruct{
    pub fn run(self, _args: Arguments, db: Database) -> Result<()> {
        let option = if self.print.is_none(){
            ListEnum::Projects( ListProjects::default() )
        }
        else{
            self.print.clone().unwrap()
        };

        use ListEnum as LE;
        return match option{
            LE::Projects(p) => 
                p.run(db),
            _ => Ok(()),
        }
    }
}

