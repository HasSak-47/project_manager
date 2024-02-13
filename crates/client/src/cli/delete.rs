use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use anyhow::{Result, anyhow};

#[derive(Args, Debug, Clone)]
pub struct DelStruct{
    name: String,
    #[clap(long, default_value="false")]
    delete_status: bool,
}

impl DelStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()> {
        Err(anyhow!("not implemented lmao"))
    }
}

