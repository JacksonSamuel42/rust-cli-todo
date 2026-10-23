mod manager;
mod models;

use manager::Manager;
use std::io::{self, Write};

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
