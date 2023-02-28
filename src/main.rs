mod options;
mod utils;
mod config;

//possible functions
mod list; mod daemon; mod add;

use std::env;
// hehehe impl trait goes brrrrr
use utils::prelude::PushAndReturn;

fn main(){
    // bitch wtf
    let arguments = env::args()
        .into_iter()
        .fold(Vec::new(), |args, arg| args.push_and_return(arg));

    options::run(if arguments.len() == 1 {options::DEFAULT_OPT}else {arguments[2].as_str()});
}
