use ly::proc::builder;
use crate::{
    desc::{Description, Descriptor},
    tags::Tag,
    TaskManager,
    utils::*
};
use serde::{Deserialize, Serialize};

#[builder(name = Task, pass = derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    #[builder(ty = Descriptor)]
    pub desc: Description,
    pub done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    #[builder(ty = u64, init = 10)]
    pub min_time: chrono::Duration,

    // minimun time needed to perform the task min_time   : time::Duration,
    #[builder(init = 1.)]
    #[builder(pass = serde(skip_serializing_if="f64::is_zero"))]
    #[builder(pass = serde(default="f64::default"))]
    pub difficulty: f64,

    #[builder(init = 1.)]
    #[builder(pass = serde(skip_serializing_if="f64::is_zero"))]
    #[builder(pass = serde(default="f64::default"))]
    pub priority: f64,

    #[builder(skip)]
    pub id : usize,

    #[builder(skip)]
    pub parent : Option<usize>,

    #[builder(ty = String, init = String::new())]
    #[builder(pass = serde(default = "String::new"))]
    pub project: Option<usize>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    childs: Vec<Task>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    tags: Vec<Tag>,
}

impl TaskTable {
    pub fn naive_task(self) -> Task{
        Task {
            priority: self.priority,
            difficulty: self.difficulty,
            desc: self.desc.naive_description(),
            done: self.done,
            min_time: self.min_time.num_minutes() as u64,
            project: String::new(),
            childs: Vec::new(),
            tags: Vec::new(),
        }
    }    
}

impl<'a> TaskManager<'a>{
    pub fn name(&self) -> &String{
        &self.get_table().desc.name
    }

    pub fn get_table(&self) -> &TaskTable{
        &self.pool.tasks[self.id]
    }

    fn __get_completion(&self, candidates: &mut Vec<TaskManager<'a>>, cdone: &mut f64, ctodo: &mut f64) {
        let mut childs = Vec::new();
        let parent = self.id;
        loop{
            match candidates.iter()
                .position(|t| t.get_table().parent.is_some_and(|p_id| p_id == parent))
            {
                Some(index) => childs.push(candidates.remove(index)),
                None => break,
            }
        }

        if self.get_table().done {
            *cdone += self.get_table().difficulty;
        }else{
            *ctodo += self.get_table().difficulty;
        } 

        for child in childs{
            child.__get_completion(candidates, cdone, ctodo);
        }
    }

    pub fn get_completion_pairs(&self) -> (f64, f64){
        let mut candidates = self.pool.get_all_tasks();
        let mut todo = 0.;
        let mut done = 0.;
        self.__get_completion(&mut candidates, &mut done, &mut todo);
        return (done, todo);
    }

    pub fn get_completion(&self) -> f64{
        let (done, todo) = self.get_completion_pairs();
        if done + todo == 0.{
            return 0.;
        }
        return done / (done + todo);
    }


}
