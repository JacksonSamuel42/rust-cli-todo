use crate::models::{Status, Task};

use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::Path;

pub struct Manager {
    tasks: Vec<Task>,
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            status: Status::PENNDING,
        }
    }

    fn done(&mut self) {
        self.status = Status::DONE
    }

    fn format(&self) -> String {
        match self.status {
            Status::DONE => format!("[X] {} - {}\n", self.id, self.description),
            Status::PENNDING => format!("[ ] {} - {}\n", self.id, self.description),
        }
    }
}

impl Manager {
    pub fn new() -> Self {
        Manager { tasks: Vec::new() }
    }

    pub fn add(&mut self, description: &str) {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task::new(id, description.into()));
    }

    pub fn list(&self) -> String {
        self.tasks
            .iter()
            .map(|task| task.format())
            .collect::<String>()
    }

    pub fn mark_as_done(&mut self, id: u32) -> bool {
        self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .map(|task| {
                task.done();
            })
            .is_some()
    }

    pub fn remove_task(&mut self, id: u32) -> bool {
        let prev_lenght = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        self.tasks.len() < prev_lenght
    }

    pub fn save_into_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for task in &self.tasks {
            let content = format!("{};{};{:?}\n", task.id, task.description, task.status);
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }

    pub fn read_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let file;
        match File::open(path) {
            Ok(f) => file = f,
            Err(_) => file = File::create(path)?,
        };
        if Path::new("task_manager.txt").exists() {}
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        match format_to_vec(contents) {
            Some(tasks) => {
                for t in tasks {
                    self.tasks.push(t);
                }
            }
            None => {
                eprintln!("Couldn't read file");
            }
        }

        Ok(())
    }
}

fn format_to_vec(content: String) -> Option<Vec<Task>> {
    let splited_task: Vec<&str> = content.trim().lines().collect();
    let mut tasks: Vec<Task> = Vec::new();
    for task in splited_task {
        let data: Vec<&str> = task.split(";").collect();
        if data.len() == 0 || data[0] == "" {
            return None;
        }
        let mut status = Status::PENNDING;
        if data[2] == "DONE" {
            status = Status::DONE
        }
        tasks.push(Task {
            id: data[0].parse::<u32>().unwrap(),
            description: data[1].to_string(),
            status: status,
        });
    }
    Some(tasks)
}
