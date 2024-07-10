mod list;
// mod rename;
mod git;
// mod features;
// mod delete;
mod init;
mod new;
// mod mark_feature;
// mod utils;


use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Subcommand, Parser, Args};

use git::GitStruct;
use init::InitStruct;
use new::NewStruct;
use project_manager_api::Handler;

use crate::{ManagerTOML, ProjectTOML};

use self::{
    list::ListStruct,
//     delete::DelStruct, features::AddFeat, git::GitStruct, mark_feature::MarkFeature, new::NewStruct
};

#[derive(Parser, Debug)]
#[clap(author="Daniel", version, about)]
pub struct Arguments{
    #[clap(short, long)]
    verbose: bool,
    #[clap(long)]
    debug: bool,
    #[clap(long)]
    manager_path: Option<PathBuf>,

    #[command(subcommand)]
    tree: Option<Tree>,
}

#[derive(Args, Default, Debug, Clone, Copy)]
struct NotDone;

impl NotDone{
    pub fn run(&self, _ : Arguments, _handler: Handler) -> Result<()>{
        Err(anyhow!("not yet implemented"))
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Tree{
    // Daemon(DaemonStruct),
    List(ListStruct),
    Init(InitStruct),
    // Delete(DelStruct),
    New(NewStruct),

    // SetParent(NotDone),
    // SetSubproject(NotDone),
    // AddFeat(AddFeat),
    // AddSubFeat(NotDone),

    // Tui(NotDone),
    // DoneFeat(NotDone),
    // Update(NotDone),

    // MarkFeature(MarkFeature),

    Git(GitStruct),
}

impl Arguments {
    pub fn debug<S: AsRef<str>>(&self, info: &S){
        let info = info.as_ref();
        if self.debug{
            println!("{}", info);
        }
    }
    
}

pub fn cli() -> Result<()> {
    // set up stuff
    let args = Arguments::parse();
    if args.tree.is_none(){
        return Err(anyhow!("no arguments given!"));
    }

    let tree = args.tree.clone().unwrap();

    let mut manager_toml = ManagerTOML::default();
    manager_toml.path = if args.manager_path.is_some(){
        args.debug(&
            format!("setting path to: {:?}", args.manager_path.clone().unwrap())
        );
        args.manager_path.clone().unwrap()
    }
    else{
        let mut dir = dirs::data_dir().unwrap();
        dir.push("project_manager");
        dir.push("projects");
        dir.set_extension("toml");
        dir
    };

    args.debug(&
        format!("handler path: {:?}", manager_toml.path)
    );
    let mut handler = Handler::new();
    handler.set_manager_io(manager_toml);
    handler.set_project_io(ProjectTOML::default());
    handler.init()?;
    

    use Tree as TR;

    match tree{
        TR::List(l) => l.run(args, handler)?,
        TR::Init(i) => i.run(args, handler)?,
        // TR::Delete(d) => d.run(args, handler)?,
        TR::New(n) => n.run(args, handler)?,
        // TR::AddFeat(f) => f.run(args, handler)?,
        TR::Git(g) => g.run(args, handler)?,
        // TR::MarkFeature(f) => f.run(args, handler)?,
        _ => NotDone::default().run(args, handler)?,
    }

    Ok(())
}
