use project_manager_api as api;
use anyhow::Result;
use rand::random;
use std::thread;
use std::sync::{Arc, Mutex, OnceLock};
use std::net::TcpListener;

use ly::log::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Event{
    id: usize,
    duration: u64,
    priority: f64,
    date: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Task{
    id: usize,
    priority: f64,
    difficulty: f64,
    done: bool,

    min_time: u64, // in seconds
    due_date: u64, // UNIX TIMESTAMP
}

impl Task {
    fn duration(&self) -> u64{
        (self.min_time as f64 * self.difficulty) as u64
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
enum EntryType{
    #[default]
    Task,
    Event,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
struct CollapsedEntry{
    id: usize,
    time: u64, // unix
    e_type: EntryType,
}

impl CollapsedEntry{
    fn new(id: usize, time: u64, e_type: EntryType) -> Self{
        Self{ id, time, e_type }
    }
    fn new_event(id: usize, time: u64) -> Self{
        Self{ id, time, e_type : EntryType::Event}
    }
    fn new_task(id: usize, time: u64) -> Self{
        Self{ id, time, e_type : EntryType::Task}
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
struct Scheduler{
    tasks: Vec<Task>,
    events: Vec<Event>,
    schedule: Vec<CollapsedEntry>,
}

impl Scheduler{
    pub const fn new() -> Self{
        Self { tasks: Vec::new(), events: Vec::new(), schedule: Vec::new(), }
    }

    pub fn add_task(&mut self, task: Task) {
        let pos = match self.tasks.binary_search_by(|t| t.priority.total_cmp( &task.priority )){
            Ok(k) => k,
            Err(r) => r,
        };
        self.tasks.insert(pos, task);
    }

    pub fn add_event(&mut self, event: Event) {
        let pos = match self.events.binary_search_by(|e| e.priority.total_cmp( &event.priority )){
            Ok(k) => k,
            Err(r) => r,
        };
        self.events.push(event);
    }

    /** returns true if the event was added false otherwise */
    fn _add_event(&mut self, current_time: &mut u64, eventc: &mut Event, taskc: &mut Task) -> bool{
        // if the event can even be started
        if *current_time < eventc.date {
            return false;
        }
        // if the task overlaps with the event skip to the event
        // and the task is more important cancel event
        if taskc.due_date < *current_time + eventc.duration && taskc.priority > eventc.priority {
                return false;
        }

        *current_time += eventc.duration;
        self.schedule.push(CollapsedEntry ::new_event(eventc.id, *current_time));
        return true;
    }

    pub fn create_schedule(&mut self) {
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

    s.create_schedule();
    Ok(())
}
