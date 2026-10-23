use std::fs::File;
use std::io::{self, Write};
use std::io::{BufReader, prelude::*};
use std::path::Path;

#[derive(Debug)]
enum Status {
    PENNDING,
    DONE,
}

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    status: Status,
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

struct Manager {
    tasks: Vec<Task>,
}

impl Manager {
    fn new() -> Self {
        Manager { tasks: Vec::new() }
    }

    fn add(&mut self, description: &str) {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task::new(id, description.into()));
    }

    fn list(&self) -> String {
        self.tasks
            .iter()
            .map(|task| task.format())
            .collect::<String>()
    }

    fn mark_as_done(&mut self, id: u32) -> bool {
        self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .map(|task| {
                task.done();
            })
            .is_some()
    }

    fn remove_task(&mut self, id: u32) -> bool {
        let prev_lenght = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        self.tasks.len() < prev_lenght
    }

    fn save_into_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for task in &self.tasks {
            let content = format!("{};{};{:?}\n", task.id, task.description, task.status);
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }

    fn read_from_file(&mut self, path: &str) -> std::io::Result<()> {
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

fn main() {
    let mut manage = Manager::new();
    manage.read_from_file("task_manager.txt").unwrap();
    print_header();
    loop {
        let mut input: String = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command = input.trim();
                let parts: Vec<&str> = command.split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "add" => {
                        let task = parts[1..].join(" ");
                        manage.add(&task);
                        manage.save_into_file("task_manager.txt").unwrap();
                        println!("Tarefa adicionada!")
                    }
                    "list" => {
                        let tasks = manage.list();
                        println!("========== Tarefas ===========");
                        println!("{}", tasks)
                    }
                    "done" => {
                        let id = parts[1].parse::<u32>().unwrap();
                        let done = manage.mark_as_done(id);
                        if !done {
                            println!("Erro ao finalizar tarefa!");
                            continue;
                        }
                        manage.save_into_file("task_manager.txt").unwrap();
                        println!("Tarefa finalizada com sucesso!")
                    }
                    "remove" => {
                        let id = parts[1].parse::<u32>().unwrap();
                        let done = manage.remove_task(id);
                        if !done {
                            println!("Erro ao remover tarefa!");
                            continue;
                        }
                        manage.save_into_file("task_manager.txt").unwrap();
                        println!("Tarefa removida com sucesso!")
                    }
                    "exit" => {
                        println!("\nSaindo do Todo List. Até mais!");
                        break;
                    }
                    _ => {
                        println!("\n[Erro] Comando desconhecido: '{}'", parts[0]);
                    }
                }
            }
            Err(error) => {
                println!("Erro ao ler o comando: {}", error);
                break;
            }
        }
    }
}

fn print_header() {
    print!("\x1B[2J\x1B[1;1H");
    println!(
        r#"
    ╔════════════════════════════════════╗
    ║           TODO LIST CLI            ║
    ╚════════════════════════════════════╝

    Comandos disponíveis:
      • add <tarefa>   - Adicionar uma nova tarefa
      • list           - Listar todas as tarefas
      • done <id>      - Marcar tarefa como concluída
      • remove <id>    - Remover uma tarefa
      • exit           - Sair do programa
    --------------------------------------
            "#
    );
}
