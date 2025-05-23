use project_manager_api as api;
use anyhow::Result;
use rand::random;
use std::thread;
use std::sync::{Arc, Mutex, OnceLock};
use std::net::TcpListener;

use ly::log::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Task{
    priority: f64,
    difficulty: f64,
    done: bool,

    min_time: u64, // in seconds
    due_date: u64, // UNIX TIMESTAMP
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
struct Scheduler{
    current_time: u64,
    delta_time: u64,
    tasks: Vec<Task>,
    schedule: Vec<Task>,
}

impl Scheduler{
    pub const fn new() -> Self{
        Self { current_time: 0, delta_time: 0, tasks: Vec::new(), schedule: Vec::new(), }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }


    pub fn slot(&mut self) {
        // reverse order
        self.tasks.sort_by(|a, b| b.due_date.cmp(&a.due_date));
        loop {
            let last = self.tasks.pop();
            if last.is_none(){
                break;
            }
            let last = last.unwrap();

            if last.due_date > self.delta_time{
                if last.due_date <= 
                continue;
            }
            else{
            }

        }
        println!("{:#?}", self.tasks);
    }
}

fn main() -> Result<()> {
    let mut s = Scheduler::new(); 
    for _ in 0..10{
        let task = Task {
            due_date : 10u64 + random::<u64>() % 4u64,
            min_time : random::<u64>() % 4u64,//60 * 15 * (random::<u64>() % (60 * 15)),
            // difficulty : (random::<u64>() % 100) as f64 / 100f64,
            priority   : (random::<u64>() % 100) as f64 / 100f64,

            ..Default::default()
        };

        s.add_task(task);
    }

    s.slot();
    Ok(())
}
