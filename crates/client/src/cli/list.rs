

use project_manager_api::Database;
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

impl ListStruct{
    pub fn run(self, _args: Arguments, _db: Database) -> Result<()> {
        let _option = if self.print.is_none(){
            ListEnum::Projects( ListProjects::default() )
        }
        else{
            self.print.clone().unwrap()
        };
        
        return Ok(())
    }
}

