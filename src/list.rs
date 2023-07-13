use super::config::*;
use super::errors::*;


fn catcher() -> Result<()>{
    // let projects = project::get_projects().unwrap();
    // for project in projects{
    //     let done_total = project.todo.done.iter().fold(0.0, |a, b| a + b.1);
    //     let todo_total = project.todo.todo.iter().fold(0.0, |a, b| a + b.1);
    //     println!("{}: {:.3} {:.3}", project.data.name, todo_total, done_total);
    // }
    Ok(())
}

pub fn list(){
    catcher().unwrap();
}
