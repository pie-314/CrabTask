# CrabTask

CrabTask is a TUI (Text User Interface) based Todo App written in Rust. It helps you organize daily tasks efficiently and interactively right from your terminal, with a focus on speed, usability, and modern Rust tooling.

---

## 🦀 Features

- **Interactive TUI**: Manage tasks in a responsive and visually appealing terminal UI.
- **Keyboard Controls**: Quickly add, edit, mark done, and delete tasks with intuitive keyboard shortcuts.
- **Persistent Storage**: Your tasks are saved and loaded automatically for each day.
- **Calendar View**: Navigate tasks by date with an integrated calendar.
- **Progress Indicator**: Visual gauge showing completion progress.
- **Cross-platform**: Runs on Linux, macOS, and Windows.
- **Open Source**: MIT licensed and open for community contributions.

---

## 🛠️ Dependencies

CrabTask leverages modern, battle-tested Rust libraries, including:

### Core Libraries

- **[ratatui](https://crates.io/crates/ratatui):**  
  A high-performance library for building rich TUIs (Text User Interfaces) in Rust.  
  Used for layout, widgets, styling, and rendering the interactive interface.

- **[crossterm](https://crates.io/crates/crossterm):**  
  Enables cross-platform terminal manipulation, handling raw mode, input events, and drawing.

- **[chrono](https://crates.io/crates/chrono):**  
  For robust date and time handling, especially for calendar and daily task features.

- **[serde_json](https://crates.io/crates/serde_json):**  
  For serializing and deserializing tasks to JSON, enabling persistent storage.

- **[color-eyre](https://crates.io/crates/color-eyre):**  
  For colorful, easy-to-read error reporting in the terminal.

### Example Code Usage (from CrabTask)

```rust
use ratatui::{
    widgets::{Block, Borders, Gauge, List, Paragraph},
    style::{Color, Stylize},
    layout::{Layout, Constraint},
    text::{Line, Span},
    DefaultTerminal, Frame,
};
use crossterm::event::{self, Event, KeyCode};
use chrono::Local;
use serde_json;
```

---

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (v1.60 or newer)

### Build from Source

```sh
git clone https://github.com/pie-314/CrabTask.git
cd CrabTask
cargo build --release
```

The binary will be available at `target/release/crabtask`.

---

## 🚀 Usage

Run CrabTask from your terminal:

```sh
./crabtask
```

### Keyboard Shortcuts

- `a` – Add a new task
- `d` – Mark selected task as done
- `r` – Remove selected task
- `↑ / ↓` – Move selection up/down
- `Esc` – Quit application

For more options and help:

```sh
./crabtask --help
```

---

## 📝 Contributing

Contributions are welcome!  
Please read our [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before submitting pull requests or issues.

---

## 📄 License

Distributed under the [MIT License](LICENSE).

---

## 🙏 Acknowledgements

- Built with [ratatui](https://github.com/tui-rs/ratatui), [crossterm](https://github.com/crossterm-rs/crossterm), and the awesome Rust ecosystem.
- Thanks to all contributors and users for your feedback and support!

---

> **CrabTask:** Organize your day, the Rusty way!
