use super::CliUtil;
use crate::error::ProjectResult;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Projects{
    #[default]
    Percentajes,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PrintType{
	Projects(Projects),
	Project(String),
	Current,
	Todo,
	Done,
}

impl Default for PrintType{
    fn default() -> Self {
        Self::Projects(Projects::default())
    }
}

#[derive(Default, Debug)]
pub struct Print();

impl CliUtil for Print{
    fn run(&self) -> ProjectResult<()>{
        Ok(())
    }

    fn add_arg(&mut self, arg: String) -> ProjectResult<()> {
        match arg.as_str(){
            "projects" => {},
            _ => {},
        }
        Ok(())
    }
}
