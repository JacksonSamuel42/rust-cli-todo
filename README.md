
# Todo List CLI

Uma aplicação de linha de comandos (CLI) desenvolvida em Rust para gerir tarefas. Permite adicionar, listar, concluir e remover tarefas, guardando os dados localmente num ficheiro de texto.

## Funcionalidades

- Adicionar novas tarefas.
- Listar todas as tarefas.
- Marcar tarefas como concluídas.
- Remover tarefas.
- Guardar as tarefas num ficheiro local.
- Carregar automaticamente as tarefas ao iniciar a aplicação.
- Validar comandos e apresentar mensagens de erro.
- Consultar os comandos disponíveis através de `help`.

## Tecnologias utilizadas

- **Rust** — linguagem de programação.
- **Cargo** — ferramenta de gestão e compilação do projeto.
- **File I/O** — leitura e escrita de ficheiros para persistência dos dados.

## Requisitos

Para executar o projeto, precisas de ter o Rust e o Cargo instalados.

Podes instalar o Rust através do site oficial:

https://www.rust-lang.org/tools/install

## Estrutura do projeto

```text
todo-list/
├── Cargo.toml
├── src/
│   └── main.rs
└── task_manager.txt
```

- `Cargo.toml`: contém os metadados e a configuração do projeto.
- `src/main.rs`: contém a implementação da aplicação.
- `task_manager.txt`: armazena as tarefas localmente e é criado automaticamente caso não exista.

## Instalação e execução

Clona o repositório e entra na pasta do projeto:

```bash
git clone <url-do-repositorio>
cd todo-list
```

Executa a aplicação com o Cargo:

```bash
cargo run
```

## Comandos disponíveis

| Comando | Descrição |
|---|---|
| `add <tarefa>` | Adiciona uma nova tarefa. |
| `list` | Lista todas as tarefas. |
| `done <id>` | Marca uma tarefa como concluída. |
| `remove <id>` | Remove uma tarefa pelo seu ID. |
| `help` | Mostra os comandos disponíveis. |
| `exit` | Encerra a aplicação. |

## Exemplos de utilização

### Adicionar tarefas

```text
> add Estudar Rust
Tarefa adicionada!

> add Aprender ownership
Tarefa adicionada!
```

### Listar tarefas

```text
> list
========== Tarefas ==========
[ ] 1 - Estudar Rust
[ ] 2 - Aprender ownership
```

### Concluir uma tarefa

```text
> done 1
Tarefa finalizada com sucesso!
```

Depois de concluir a tarefa, a listagem apresenta:

```text
[X] 1 - Estudar Rust
[ ] 2 - Aprender ownership
```

### Remover uma tarefa

```text
> remove 2
Tarefa removida com sucesso!
```

### Sair da aplicação

```text
> exit
Saindo do Todo List. Até mais!
```

## Armazenamento dos dados

As tarefas são guardadas no ficheiro `task_manager.txt`, utilizando o seguinte formato:

```text
<id>;<descrição>;<estado>
```

Exemplo:

```text
1;Estudar Rust;DONE
2;Aprender ownership;PENDING
```

Os estados possíveis são:

- `PENDING` — tarefa pendente.
- `DONE` — tarefa concluída.

Os dados são carregados quando a aplicação inicia e atualizados após operações que alteram as tarefas.

## Organização do código

- **`Status`**: enumeração que representa o estado de uma tarefa.
- **`Task`**: estrutura que representa uma tarefa, com ID, descrição e estado.
- **`Manager`**: gere a coleção de tarefas e as operações disponíveis.
- **`parse_tasks`**: interpreta e valida os dados carregados do ficheiro.
- **`execute_command`**: interpreta e executa os comandos introduzidos pelo utilizador.
- **`print_header`**: apresenta o cabeçalho e a ajuda da aplicação.

## Tratamento de erros

A aplicação inclui validações para reduzir falhas durante a execução:

- Impede a criação de tarefas com descrições vazias.
- Valida os IDs introduzidos nos comandos.
- Verifica o formato dos dados carregados.
- Deteta estados inválidos e IDs duplicados no ficheiro.
- Apresenta mensagens de erro quando ocorrem problemas de leitura ou escrita.

## Melhorias futuras

Algumas funcionalidades que podem ser implementadas em versões futuras:

- [X] Separar o código em módulos.
- [ ] Adicionar testes unitários.
- [ ] Editar descrições de tarefas.
- [ ] Filtrar tarefas pendentes e concluídas.
- [ ] Ordenar tarefas por ID ou estado.
- [ ] Adicionar datas de criação e conclusão.
- [ ] Utilizar JSON ou uma base de dados para armazenamento.
- [ ] Criar uma interface gráfica.

## Objetivo do projeto

Este projeto serve como exercício prático de aprendizagem de Rust, explorando estruturas, enumerações, métodos, vetores, tratamento de erros, manipulação de ficheiros e interação com o terminal.
