# CrabTask

CrabTask is a lightweight, efficient task management application built with Rust. Designed for developers and productivity enthusiasts, CrabTask provides a streamlined way to organize, track, and manage your daily tasks directly from your terminal.

## Features

- **Fast & Reliable**: Built with Rust for speed and safety.
- **Intuitive CLI**: Simple command-line interface for quick task management.
- **Task Prioritization**: Easily set priorities and deadlines.
- **Persistence**: Your tasks are saved and loaded automatically.
- **Cross-platform**: Works on Linux, macOS, and Windows.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or newer)

### Build from Source

```sh
git clone https://github.com/pie-314/CrabTask.git
cd CrabTask
cargo build --release
```

The compiled binary will be available in `target/release/crabtask`.

## Usage

Run CrabTask from your terminal:

```sh
./crabtask [OPTIONS] [SUBCOMMAND]
```

### Common Commands

- `add "Task description"`: Add a new task
- `list`: Show all tasks
- `done <task_id>`: Mark a task as completed
- `delete <task_id>`: Remove a task
- `edit <task_id> "New description"`: Edit a task's description

For the full list of commands and options:

```sh
./crabtask --help
```

## Example

```sh
./crabtask add "Write documentation"
./crabtask list
./crabtask done 2
```

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) guide before submitting any changes.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

## License

CrabTask is released under the [MIT License](LICENSE).

## Acknowledgements

- Inspired by the Rust community and CLI productivity tools.
- Thanks to all contributors and users for their feedback and support.

---

**CrabTask**: Organize your day, the Rusty way!
