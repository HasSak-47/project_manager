use super::config::{Project, Manager};
use super::daemon::daemon;
use super::list::list;
use super::add::add;

struct Option {
    name: &'static str,
    func: fn(),
}

const OPTIONS: [Option; 3] = [
    Option{name: "init_daemon"  , func: daemon},
    Option{name: "list_projects", func: list},
    Option{name: "add_project"  , func: add},
];

pub const DEFAULT_OPT: &'static str= &OPTIONS[1].name;

pub fn run(opt: &str){
    for option in OPTIONS{
        if option.name == opt{
            //lmao
            let f = option.func;
            f();
        }
    } 
}
