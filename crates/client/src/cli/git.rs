
use super::Params;
use clap::Args;
use project_manager_api::error::ProjectResult;

#[derive(Args, Debug, Default, Clone)]
pub struct GitStruct{
    #[clap(allow_hyphen_values=true)]
    args: Vec<String>,
}

impl GitStruct{
    pub fn run(&self, _params: Params) -> ProjectResult<()>{
        for arg in &self.args{
            println!("{arg}");
        }
        Ok(())
    }
}


