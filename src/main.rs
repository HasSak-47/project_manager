mod options;
mod utils;
mod config;
mod errors;
mod constats;

//possible functions
mod list;
mod daemon;
mod add;

use std::env;
use serde::Deserialize;
// hehehe impl trait goes brrrrr
use utils::prelude::PushAndReturn;

fn main(){
    use config::project::*;

    let a : Project = toml::from_str(
"
[id]
name = \"test\"
version = \"0.0.0\"

[features.done]
wacky =  0.0
[features.todo]
wacky =  0.0

[front.done]
wacky =  0.0
[front.todo]
wacky =  0.0

[middle.done]
wacky =  0.0
[middle.todo]
wacky =  0.0

[back.done]
wacky =  0.0
[back.todo]
wacky =  0.0

").unwrap();

    // bitch wtf
    // let arguments = env::args()
    //     .into_iter()
    //     .fold(Vec::new(), |args, arg| args.push_and_return(arg));

    // options::run(if arguments.len() == 1 {options::DEFAULT_OPT} else {arguments[1].as_str()});
}
