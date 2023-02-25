use super::configs::{config, project, status};
use crossterm::terminal::size as get_termina_size;
use toml;

#[derive(Debug)]
struct Completion{
    pub name: String,
    pub perc: f64,
}

impl Completion{
    fn new(name: String, perc: f64) -> Self {Self{name, perc}}
}

fn format_f64(name: &String, val: f64, longest_name: usize, max_width: usize) -> String{
    let width = max_width - (longest_name + 12); 
    let a = (width as f64 * val) as usize;
    let b = width - a;
    format!("{name:longest_name$}: [{:#<a$}{: <b$}] {:>6.2}%", "", "", val * 100.0)
    
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
    let width = get_termina_size().unwrap().0 as usize;
    for project in p_data{
        println!("{}", format_f64(&project.name, project.perc, longest_name, width));
        //println!(format_f64(project.name, Project.longest_name, ))
        // println!("{:longest_name$}: {}", project.name, format_f64(project.perc))
    }
}
