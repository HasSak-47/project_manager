mod print;
use print::Print;

use crate::error::{ProjectResult, ProjectError};

pub trait CliUtil where
{
    fn run(&self) -> ProjectResult<()>;
    fn add_arg(&mut self, arg: String) -> ProjectResult<()> { Ok(()) }
}

pub fn determine_util<S: AsRef<str>>(name: S) -> ProjectResult<Box<dyn CliUtil>>{
    let name = name.as_ref();
    match name{
        "print" => Ok(Box::new(Print::default())),
        _ => Err(ProjectError::CliOptionUnknown),
    }
}

pub fn run_cli(mut args: Vec<String>) -> ProjectResult<()>{
    let util = if args.len() == 0{
        Box::new(Print::default())
    }
    else{
        let mut util = determine_util(&args[0])?;
        args.pop();
        for arg in args{
            util.add_arg(arg)?;
        }
        util
    };

    util.run()?;
    Ok(())
}
