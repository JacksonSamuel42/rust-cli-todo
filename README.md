
# Todo List CLI

Uma aplicação de linha de comandos (CLI) desenvolvida em Rust para gerir tarefas. Permite adicionar, listar, concluir e remover tarefas, guardando os dados localmente num ficheiro JSON.

## Funcionalidades

- Adicionar novas tarefas, com a data de criação.
- Listar todas as tarefas.
- Filtrar tarefas por estado (`--status`) e por ID (`--id`).
- Marcar tarefas como concluídas.
- Editar a descrição de uma tarefa.
- Remover tarefas.
- Guardar as tarefas num ficheiro JSON local.
- Carregar automaticamente as tarefas ao iniciar a aplicação.
- Validar comandos e apresentar mensagens de erro.
- Consultar os comandos disponíveis através de `help`.

## Tecnologias utilizadas

- **Rust** — linguagem de programação.
- **Cargo** — ferramenta de gestão e compilação do projeto.
- **serde** / **serde_json** — serialização das tarefas para JSON.
- **chrono** — data e hora de criação das tarefas.
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
│   ├── main.rs
│   ├── manager.rs
│   └── models.rs
└── task_manager.json
```

- `Cargo.toml`: contém os metadados e a configuração do projeto.
- `src/main.rs`: ponto de entrada, leitura de comandos e interface de terminal.
- `src/manager.rs`: operações sobre as tarefas e persistência em ficheiro.
- `src/models.rs`: tipos `Task` e `Status`.
- `task_manager.json`: armazena as tarefas localmente e é criado automaticamente caso não exista.

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

Para correr os testes:

```bash
cargo test
```

## Comandos disponíveis

| Comando | Descrição |
|---|---|
| `add <tarefa>` | Adiciona uma nova tarefa. |
| `list [filtros]` | Lista as tarefas, opcionalmente filtradas. |
| `done <id>` | Marca uma tarefa como concluída. |
| `edit <id> <nova descrição>` | Edita a descrição de uma tarefa. |
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
========== Tarefas ===========
[ ] 1 - Estudar Rust - 05-11-2026 14:30
[ ] 2 - Aprender ownership - 05-11-2026 14:30
```

#### Filtrar a listagem

O `list` aceita filtros opcionais no formato `--filtro=valor`, combinados com **E** (têm de bater todos):

| Filtro | Valores |
|---|---|
| `--status` | `pending` ou `done` (sem distinção de maiúsculas) |
| `--id` | ID da tarefa |

```text
> list --status=done
========== Tarefas ===========
[X] 1 - Estudar Rust - 05-11-2026 14:30

> list --status=pending --id=2
========== Tarefas ===========
[ ] 2 - Aprender ownership - 05-11-2026 14:30
```

Um filtro inválido (estado desconhecido, ID não numérico, flag inexistente ou falta do `=`) apresenta uma mensagem de erro e não altera os dados.

### Concluir uma tarefa

```text
> done 1
Tarefa finalizada com sucesso!
```

Depois de concluir a tarefa, a listagem apresenta:

```text
[X] 1 - Estudar Rust - 05-11-2026 14:30
[ ] 2 - Aprender ownership - 05-11-2026 14:30
```

### Editar uma tarefa

```text
> edit 2 Aprender ownership e borrowing
Tarefa atualizada com sucesso!
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

As tarefas são guardadas em `task_manager.json`, como um array de objetos:

```json
[
  {
    "id": 1,
    "description": "Estudar Rust",
    "status": "DONE",
    "created_at": "2026-11-05T14:30:00Z"
  },
  {
    "id": 2,
    "description": "Aprender ownership",
    "status": "PENDING",
    "created_at": "2026-11-05T14:30:00Z"
  }
]
```

- `id` — identificador da tarefa.
- `description` — descrição livre (pode conter `;`, aspas ou acentos; o JSON escapa o que for preciso).
- `status` — `PENDING` (pendente) ou `DONE` (concluída).
- `created_at` — data e hora de criação, em RFC 3339 (UTC).

Um ficheiro vazio é tratado como lista vazia. Os dados são carregados quando a aplicação inicia e atualizados após cada operação que altere as tarefas.

## Organização do código

- **`Status`**: enumeração que representa o estado de uma tarefa.
- **`Task`**: estrutura que representa uma tarefa, com ID, descrição, estado e data de criação.
- **`Manager`**: gere a coleção de tarefas e as operações disponíveis (`add`, `add_at`, `list`, `mark_as_done`, `update`, `remove_task`).
- **`Task::new`**: recebe o instante de criação, para que o relógio não fique escondido dentro do construtor.
- **`serde`**: `#[derive(Serialize, Deserialize)]` nos modelos faz a leitura e escrita do JSON.
- **`execute_command`**: interpreta e executa os comandos introduzidos pelo utilizador.
- **`print_header`**: apresenta o cabeçalho e a ajuda da aplicação.

## Tratamento de erros

A aplicação inclui validações para reduzir falhas durante a execução:

- Valida os IDs introduzidos nos comandos.
- Rejeita filtros inválidos (estado desconhecido, ID não numérico, flag inexistente).
- Apresenta um erro quando o JSON do ficheiro não pode ser lido.
- Apresenta mensagens de erro quando ocorrem problemas de leitura ou escrita.

## Melhorias futuras

Algumas funcionalidades que podem ser implementadas em versões futuras:

- [X] Separar o código em módulos.
- [X] Adicionar testes unitários.
- [X] Editar descrições de tarefas.
- [X] Filtrar tarefas pendentes e concluídas.
- [X] Adicionar data de criação às tarefas.
- [X] Utilizar JSON para armazenamento.
- [ ] Adicionar data de conclusão.
- [ ] Ordenar tarefas por ID ou estado.
- [ ] Criar uma interface gráfica.

## Objetivo do projeto

Este projeto serve como exercício prático de aprendizagem de Rust, explorando estruturas, enumerações, métodos, vetores, tratamento de erros, serialização com `serde`, datas com `chrono`, manipulação de ficheiros e interação com o terminal.
