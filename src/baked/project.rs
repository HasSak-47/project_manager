use crate::config::project;

#[derive(Default)]
pub struct Project{
    pub project: project::ProjectInfo,
    pub subproj: Vec<usize>,
    todo: Vec<project::Feature>,
    done: Vec<project::Feature>,
}

impl Project{
    fn bake_prject(p: project::Project) -> Vec<Self>{
        let mut v = Vec::new();
        let mut s = Self::default();

        s.project = p.project;
        s.done = p.done;
        s.todo = p.todo;

        // flatten three

        v
    }
}


