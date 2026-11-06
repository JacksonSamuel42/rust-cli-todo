use crate::models::{Status, Task};

use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::Error;

pub struct Manager {
    tasks: Vec<Task>,
}

impl Task {
    fn new(id: u32, description: String, created_at: DateTime<Utc>) -> Self {
        Task {
            id,
            description,
            status: Status::PENDING,
            created_at: created_at,
            done_at: None,
        }
    }

    fn done(&mut self, done_at: DateTime<Utc>) {
        self.status = Status::DONE;
        self.done_at = Some(done_at)
    }

    fn format(&self) -> String {
        match self.status {
            Status::DONE => format!(
                "[X] {} - {} - {}\n",
                self.id,
                self.description,
                match self.done_at {
                    Some(date) => date.format("%d-%m-%Y %H:%M").to_string(),
                    None => "done".to_string(),
                }
            ),
            Status::PENDING => format!(
                "[ ] {} - {} - {}\n",
                self.id,
                self.description,
                self.created_at.format("%d-%m-%Y %H:%M").to_string()
            ),
        }
    }
}

impl Manager {
    pub fn new() -> Self {
        Manager { tasks: Vec::new() }
    }

    fn next_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn add(&mut self, description: &str) {
        let id = self.next_id();
        self.tasks
            .push(Task::new(id, description.into(), Utc::now()));
    }

    #[cfg(test)]
    pub fn add_at(&mut self, description: &str, created_at: DateTime<Utc>) {
        let id = self.next_id();
        self.tasks
            .push(Task::new(id, description.into(), created_at));
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

    pub fn list(&mut self, filters: &str) -> Result<String, std::io::Error> {
        let parts = filters.trim().split_whitespace().collect::<Vec<&str>>();

        let mut status: Option<Status> = None;
        let mut id: Option<u32> = None;

        for part in parts {
            if part.is_empty() {
                continue;
            }

            let (flag, value) = part.split_once("=").ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "falta o '=' no filtro",
            ))?;

            match flag {
                "--status" => match value.to_uppercase().as_str() {
                    "PENDING" => status = Some(Status::PENDING),
                    "DONE" => status = Some(Status::DONE),
                    _ => {
                        return Err(Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "invalid status",
                        ));
                    }
                },
                "--id" => {
                    let parsed_id = value
                        .parse::<u32>()
                        .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "id invalido"))?;
                    id = Some(parsed_id)
                }
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "there is an invalid filter",
                    ));
                }
            }
        }

        self.tasks.sort_by_key(|t| t.id);
        let result = self
            .tasks
            .iter()
            .filter(|t| status.is_none_or(|s| t.status == s) && id.is_none_or(|id| t.id == id))
            .map(|t| t.format())
            .collect();

        Ok(result)
    }

    pub fn mark_as_done(&mut self, id: u32) -> bool {
        self.mark_as_done_at(id, Utc::now())
    }

    pub fn mark_as_done_at(&mut self, id: u32, done_at: DateTime<Utc>) -> bool {
        self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .map(|task| task.done(done_at))
            .is_some()
    }

    pub fn remove_task(&mut self, id: u32) -> bool {
        let prev_lenght = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        self.tasks.len() < prev_lenght
    }

    pub fn save_into_file(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(&file, &self.tasks)?;
        Ok(())
    }

    pub fn read_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        if file.metadata()?.len() == 0 {
            return Ok(());
        }

        self.tasks = serde_json::from_reader(file)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn fixed_date() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 5, 14, 30, 0).unwrap()
    }

    fn fixed_done_date() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 5, 15, 0, 0).unwrap()
    }

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
        manager.add_at("Passar no teste", fixed_date());
        manager.add_at("Estudar rust", fixed_date());
        let expected =
            "[ ] 1 - Passar no teste - 05-11-2026 14:30\n[ ] 2 - Estudar rust - 05-11-2026 14:30\n";
        assert_eq!(manager.list("").unwrap(), expected)
    }

    #[test]
    fn should_list_tasks_with_filter() {
        let mut manager = Manager::new();
        manager.add_at("Passar no teste", fixed_date());
        manager.add_at("Estudar rust", fixed_date());

        assert_eq!(
            manager.list("").unwrap(),
            "[ ] 1 - Passar no teste - 05-11-2026 14:30\n[ ] 2 - Estudar rust - 05-11-2026 14:30\n"
        );
        manager.mark_as_done_at(2, fixed_done_date());
        assert_eq!(
            manager.list("--status=done --id=2").unwrap(),
            "[X] 2 - Estudar rust - 05-11-2026 15:00\n"
        );
        assert_eq!(
            manager.list("--status=pending").unwrap(),
            "[ ] 1 - Passar no teste - 05-11-2026 14:30\n"
        )
    }

    #[test]
    fn should_return_error_for_invalid_filter() {
        let mut manager = Manager::new();
        manager.add("Passar no teste");

        assert!(manager.list("--status=done2").is_err());
        assert!(manager.list("--id=abc").is_err());
        assert!(manager.list("--unknown=1").is_err())
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
        assert!(manager.list("").unwrap().is_empty())
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
