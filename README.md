# CrabTask

**CrabTask** is a modern, terminal-based task management application written in Rust. It leverages a TUI (Terminal User Interface) to provide a robust, interactive, and visually appealing way to manage your daily to-dos, with features such as calendar navigation, task highlighting, and progress tracking—all from your terminal.

---

## Features

- **Task Management**: Add, list, and manage your daily tasks, organized by date.
- **Interactive TUI**: Built with [ratatui](https://github.com/ratatui-org/ratatui), offering a smooth and keyboard-navigable interface.
- **Calendar Integration**: See your current month's calendar, with today highlighted.
- **Persistent Storage**: All tasks are stored in a local JSON file (`data/todo_data.json`), ensuring your data persists across sessions.
- **Task Details**: Each task can have a title, due date, priority, notes, and completion status.
- **Keyboard Controls**: Navigate tasks, mark them complete, add new tasks, and more using simple key bindings.

---

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- A terminal emulator that supports UTF-8 and ANSI colors

### Installation

1. **Clone the repository**
   ```sh
   git clone https://github.com/pie-314/CrabTask.git
   cd CrabTask
   ```

2. **Build the project**
   ```sh
   cargo build --release
   ```

3. **Run the application**
   ```sh
   cargo run
   ```

   On first run, a `data/todo_data.json` file will be created if it does not exist.

---

## Usage

Once running, CrabTask presents a TUI with your daily tasks and a calendar view.

### Key Bindings

| Key         | Action                         |
|-------------|-------------------------------|
| Up/Down     | Navigate tasks                 |
| `a`         | Add a new task (TUI/legacy)   |
| `d`         | Delete selected task           |
| `m`         | Mark task as complete          |
| `q` or `Q`  | Quit application              |

> *Note: Some keyboard shortcuts may be available only in the legacy interface.*

### Task Data Format

Tasks are stored in a JSON file, each as a `Todo` object:
```json
{
  "id": "uuid",
  "title": "Buy groceries",
  "completed": false,
  "due_date": "2025-08-20",
  "priority": "high",
  "notes": "Buy milk, eggs, bread"
}
```

---

## Project Structure

- `src/main.rs`  
  The main entrypoint with the TUI interface, event loop, and rendering logic.
- `src/types/mod.rs`  
  Defines core types like `AppState` and `Todo`.
- `src/json_parser/mod.rs`  
  Functions for loading and parsing task data from JSON.
- `src/main_old.rs`  
  (Legacy) Early implementation with basic CLI logic. Kept for reference.
- `data/todo_data.json`  
  Local persistent storage for tasks.

---


## Contributing

Pull requests and issues are welcome! If you have suggestions for additional features, improvements, or bug reports, please open an issue or PR.

---

## License

This project is licensed under the MIT License.

---

## Acknowledgements

- Inspired by classic CLI productivity tools and the Rust community.
- Thanks to [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI library.

---

**Happy tasking! 🦀**

**NOTE: MANY FEATURES ARE NOT YET IMPLEMENTED AND I AM WORKING ON IT.**
**NOTE: BUILD MIGHT NOT WORK, RUN IN DEBUG FOR NOW.**
