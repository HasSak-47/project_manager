use super::configs::{config::Config, project::Project};
use std::time:: *;

enum EventType{
    Commit,
    Push,
    CommitPush,
}

struct Event{
    project_id: usize,
    event_type: EventType,

    time: SystemTime,
}

pub struct Local{
    pub countdown   : Duration,
    pub add_all     : bool,
    pub force_commit: bool,
}

pub struct Remote{
    pub push        : bool,
    pub countdown   : Duration,
    pub local_commit: bool,
    pub force_push  : bool,
}

pub struct ProjectManager{
    id    : usize,
    name  : String,
    folder: String,

    last_commit: SystemTime,
    last_push  : SystemTime,


    local : Local,
    remote: Remote,
}

impl ProjectManager{
    pub fn new(name: String, folder: String, local: Local, remote: Remote) -> Self{
        ProjectManager { id: 0, name, folder, last_commit: SystemTime::now(), last_push: SystemTime::now(), local, remote }
    }

    fn next_events(&self) -> Vec<Event>{ 
        let mut v = Vec::new();

        let commit_time = self.last_commit +  self.local.countdown;
        if self.remote.push == false{
            v.push(Event { project_id: self.id, event_type: EventType::CommitPush, time: commit_time});
            return v;
        }

        let push_time = self.last_push + self.remote.countdown;
        if commit_time == push_time{
            v.push(Event { project_id: self.id, event_type: EventType::CommitPush, time: commit_time});
        }
        else {
            v.push(Event { project_id: self.id, event_type: EventType::Commit, time: commit_time});
            v.push(Event {
                project_id: self.id,
                event_type: if self.remote.local_commit == true {EventType::CommitPush} else {EventType::Push},
                time: push_time
            });
        }

        v
    }

    pub fn commit(&self){} 
    pub fn push(&self){} 
}

fn get_duration(s: &String) -> std::time::Duration{
    let parts : Vec<&str> = s.split(':').collect();
    // would be really funny that it crashes
    let h = u64::from_str_radix(parts[0], 10).unwrap();
    let m = u64::from_str_radix(parts[1], 10).unwrap();

    Duration::from_secs((m + (60 * h)) * 60)
}

impl From<&Project> for ProjectManager{
    fn from(p: &Project) -> ProjectManager{
        ProjectManager::new(p.project.name.clone(), p.project.folder.clone(),
            Local {
                countdown: get_duration(&p.local.countdown),
                add_all: p.local.add_all, 
                force_commit: p.local.force_commit,
            },
            Remote {
                push: p.remote.push,
                countdown:  get_duration(&p.remote.countdown),
                local_commit: p.remote.local_commit,
                force_push: p.remote.force_push,
            }
        )
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
// should be checked in case of any bugss
pub fn main(config: Config, projects: Vec<Project>){
    let managers : Vec<ProjectManager> = projects.iter().fold(Vec::new(), |mut vec, p| {
        vec.push(ProjectManager::from(p));
        vec.last_mut().unwrap().id = vec.len() -1;
        vec
    });

    loop {
        let mut events : Vec<Event> = managers.iter().fold(Vec::new(), |mut v, manager|{
            for event in manager.next_events(){v.push(event);}
            v
        });
        events.sort_by(|a,b|{
            if a.time < b.time{
                std::cmp::Ordering::Greater
            }
            else{
                std::cmp::Ordering::Less
            }
        });

        while events.len() != 0 {
            let now = SystemTime::now();
            let fut = events.last().unwrap().time;
            if fut > now{
                let delta_time = fut.duration_since(now).unwrap();
                std::thread::sleep(delta_time);
            };
            println!("commit! project {}", managers[events.last().unwrap().project_id].name);
            events.pop();
        }
        break;
    }
}
