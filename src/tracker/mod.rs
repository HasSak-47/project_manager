use super::configs::{config, project, status};

#[derive(Debug)]
struct Completion{
    pub name: String,
    pub perc: f64,
}

impl Completion{
    fn new(name: String, perc: f64) -> Self {Self{name, perc}}
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
}
