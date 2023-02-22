use super::configs::{config, project, status};
use toml;

#[derive(Debug)]
struct Completion{
    pub name: String,
    pub perc: f64,
}

impl Completion{
    fn new(name: String, perc: f64) -> Self {Self{name, perc}}
}

fn format_f64(mut p: f64) -> String{
    let c = (p * 10.) as usize;
    let r = 10 - c;
    let mut outs = String::new();

    for _ in 0..c{
        outs.push('#')
    };
    for _ in c..10{
        outs.push(' ')
    };

    p *= 100.0;
    format!("[{outs}] {p:6.2} %")
}

pub fn main(_: config::Config, projects: Vec<project::Project>) {
    let mut p_data : Vec<Completion> = Vec::new(); 
    for project in &projects{
        let status : status::Status = match std::fs::read(&project.project.todo_path){
            Ok(k) => {
                let file_data = String::from_utf8(k).unwrap();
                toml::from_str(file_data.as_str()).unwrap()
            },
            Err(_) => {
                std::fs::write(&project.project.todo_path, status::DEFAULT_STATUS).unwrap();
                status::Status {done: None, todo: None}
            },
        };
        p_data.push(Completion::new(project.project.name.clone(), status.analyze()));
    } 

    //formating goes here
    let longest_name = p_data.iter().max_by(|a, b| {a.name.len().cmp(&b.name.len())}).unwrap().name.len();
    for project in p_data{
        println!("{:longest_name$}: {}", project.name, format_f64(project.perc))
    }
}
