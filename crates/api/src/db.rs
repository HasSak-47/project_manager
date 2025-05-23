use std::{collections::HashMap, fmt::format, ops::DerefMut, path::Path, rc::Rc};

use anyhow::{anyhow, Result};
use rusqlite::{backup::Backup, params, Connection};

pub struct DatabaseNode {
    pub con: Connection,
    pub externals: HashMap<usize, Rc<DatabaseNode>>,
}

pub struct DatabaseGraph {
    pub root: DatabaseNode,
    pub nodes: HashMap<String, Rc<DatabaseNode>>,
}

pub struct Project{
    pub local_id: usize,
    pub name: String,
}

pub struct Task{
    pub local_id: usize,
    pub parent: Option<usize>,
    pub name: String,
}

impl DatabaseNode {
    pub fn new() -> Result<Self> {
        let con = Connection::open_in_memory()?;
        con.execute_batch(
            "BEGIN;
            CREATE TABLE Project(
                local_id INTEGER NOT NULL PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE Task(
                id INTEGER NOT NULL PRIMARY KEY,
                project INTEGER NOT NULL
                name TEXT NOT NULL
            );
            CREATE TABLE ExternalDatabase(
                local_id INTEGER NOT NULL PRIMARY KEY,
                location TEXT NOT NULL
            );
            CREATE TABLE SubprojectRel(parent_id INTEGER NOT NULL, child_path TEXT NOT NULL);
            COMMIT;",
        )?;

        return Ok(Self {
            con,
            externals: HashMap::new(),
        });
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let con = Connection::open(path)?;
        return Ok(Self {
            con,
            externals: HashMap::new(),
        });
    }

    pub fn load_databases(&mut self) -> Result<Vec<Rc<DatabaseNode>>> {
        let mut stmt = self
            .con
            .prepare("SELECT local_id, location FROM ExternalDatabase")?;
        return Ok(stmt
            .query_map([], |row| {
                let id: usize = row.get(0)?;
                let location: String = row.get(1)?;
                Ok((id, location))
            })?
            .into_iter()
            .filter_map(|child| {
                if child.is_err() {
                    return None;
                }
                let (id, location) = child.as_ref().unwrap();
                if self.externals.contains_key(&id) {
                    return None;
                }

                return DatabaseNode::from_file(format!("./testing/{location}"))
                    .ok()
                    .and_then(|a| Some(Rc::new(a)));
            })
            .collect());
    }

    pub fn unload_database(&mut self, local_id: usize) -> Result<Rc<DatabaseNode>> {
        self.externals.remove(&local_id);
        todo!();
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut dst = Connection::open(path)?;
        let backup = Backup::new(&self.con, &mut dst)?;
        backup.run_to_completion(5, std::time::Duration::from_millis(1), None)?;

        Ok(())
    }

    pub fn insert_project<S>(&mut self, name: S) -> Result<usize>
    where
        S: AsRef<str>,
    {
        return Ok(self.con.execute(
            " INSERT INTO Project (name) VALUES (?1) ",
            params![name.as_ref()],
        )?);
    }
    pub fn insert_subproject<S>(&mut self, parent_id: usize, child_path: S) -> Result<usize>
    where
        S: AsRef<str>,
    {
        return Ok(self.con.execute(
            "INSERT INTO SubprojectRel (parent_id, child_path) VALUES (?1, ?2) ",
            params![parent_id, child_path.as_ref()],
        )?);
    }

    pub fn insert_external_database<S>(&mut self, location: S) -> Result<usize>
    where
        S: AsRef<str>,
    {
        return Ok(self.con.execute(
            " INSERT INTO ExternalDatabase (location) VALUES (?1) ",
            params![location.as_ref()],
        )?);
    }
}

impl DatabaseGraph {
    pub fn new() -> Result<Self> {
        return Ok(Self {
            root: DatabaseNode::new()?,
            nodes: HashMap::new(),
        });
    }

    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        return Ok(Self {
            root: DatabaseNode::from_file(p)?,
            nodes: HashMap::new(),
        });
    }
}
