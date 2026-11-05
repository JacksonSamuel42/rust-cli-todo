use crate::models::{Status, Task};

use std::fs::File;
use std::io::{BufReader, Error, prelude::*};
use std::path::Path;

pub struct Manager {
    tasks: Vec<Task>,
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            status: Status::PENDING,
        }
    }

    fn done(&mut self) {
        self.status = Status::DONE
    }

    fn format(&self) -> String {
        match self.status {
            Status::DONE => format!("[X] {} - {}\n", self.id, self.description),
            Status::PENDING => format!("[ ] {} - {}\n", self.id, self.description),
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

    pub fn update(&mut self, id: u32, description: &str) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| format!("task with id {} not found", id))?;

        task.description = description.to_string();

        Ok(())
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
        if Path::new(path).exists() {}
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let tasks = format_to_vec(&contents)?;
        self.tasks = tasks;

        // match format_to_vec(&contents) {
        //     Ok(tasks) => self.tasks = tasks,
        //     Err(error) => eprintln!("Error parsing tasks: {}", error),
        // }

        Ok(())
    }
}

fn format_to_vec(content: &str) -> std::io::Result<Vec<Task>> {
    let splited_task: Vec<&str> = content.trim().lines().collect();
    let mut tasks: Vec<Task> = Vec::new();
    for task in splited_task {
        let data: Vec<&str> = task.splitn(3, ";").collect();

        if data.len() != 3 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid task format",
            ));
        }

        let id = data[0]
            .parse::<u32>()
            .map_err(|err| Error::new(std::io::ErrorKind::InvalidData, err))?;

        let status = match data[2] {
            "PENDING" => Status::PENDING,
            "DONE" => Status::DONE,
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Status not found",
                ));
            }
        };

        tasks.push(Task {
            id: id,
            description: data[1].to_string(),
            status: status,
        });
    }
    Ok(tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_new_task() {
        let mut manager = Manager::new();
        manager.add("Passar no teste");
        let tasks = manager.tasks;
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[0].description, "Passar no teste");
        assert_eq!(tasks[0].status, Status::PENDING)
    }

    #[test]
    fn should_update_task() {
        let mut manager = Manager::new();
        manager.add("Task 1");
        manager.add("Task 2");

        assert_eq!(manager.tasks.len(), 2);

        manager.update(2, "Task 2 updated").unwrap();
        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.tasks[0].description, "Task 1");
        assert_eq!(manager.tasks[1].description, "Task 2 updated");
    }

    #[test]
    fn should_return_error_when_task_does_not_exist() {
        let mut manager = Manager::new();
        let update = manager.update(999, "new task");

        assert!(update.is_err());
    }

    #[test]
    fn list_tasks() {
        let mut manager = Manager::new();
        manager.add("Passar no teste");
        manager.add("Estudar rust");
        let expected = "[ ] 1 - Passar no teste\n[ ] 2 - Estudar rust\n";
        assert_eq!(manager.list(), expected)
    }

    #[test]
    fn should_generate_sequential_ids() {
        let mut manager = Manager::new();
        manager.add("Task 1");
        manager.add("Task 2");
        manager.add("Task 3");

        assert_eq!(
            manager.tasks.iter().map(|t| t.id).collect::<Vec<u32>>(),
            vec![1, 2, 3]
        )
    }

    //     #[test]
    //     fn should_not_reuse_removed_ids() {
    //         let mut manager = Manager::new();
    //         manager.add("Task 1");
    //         manager.add("Task 2");
    //         manager.remove_task(2);
    //         manager.add("Task 3");
    //
    //         assert_eq!(manager.tasks.last().unwrap().id, 3)
    //     }

    #[test]
    fn should_remove_only_the_requested_task() {
        let mut manager = Manager::new();
        manager.add("Task 1");
        manager.add("Task 2");
        manager.add("Task 3");

        assert!(manager.remove_task(2));

        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.tasks[0].id, 1);
        assert_eq!(manager.tasks[1].id, 3)
    }

    #[test]
    fn mark_task_as_done() {
        let mut manager = Manager::new();
        manager.add("Passar no teste");
        assert!(manager.mark_as_done(1));
        assert_eq!(manager.tasks[0].status, Status::DONE)
    }

    #[test]
    fn mark_nonexistent_task_as_done() {
        let mut manager = Manager::new();

        assert!(!manager.mark_as_done(999))
    }

    #[test]
    fn remove_task() {
        let mut manager = Manager::new();
        manager.add("Passar no teste");
        assert!(manager.remove_task(1));
        assert!(manager.list().is_empty())
    }

    #[test]
    fn remove_nonexistent_task() {
        let mut manager = Manager::new();

        assert!(!manager.remove_task(999))
    }

    #[test]
    fn should_save_and_load_tasks() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();

        let mut manager = Manager::new();

        manager.add("Learn Rust");
        manager.add("Build a project");
        manager.mark_as_done(1);

        manager.save_into_file(path).unwrap();

        let mut new_manager = Manager::new();
        new_manager.read_from_file(path).unwrap();

        assert_eq!(new_manager.tasks.len(), 2);
        assert_eq!(new_manager.tasks[0].id, 1);
        assert_eq!(new_manager.tasks[0].status, Status::DONE);
        assert_eq!(new_manager.tasks[1].id, 2);
        assert_eq!(new_manager.tasks[1].status, Status::PENDING);
    }
}
