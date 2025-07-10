# KLA (Kommand Line Automation)

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A **Playwright equivalent for beautiful CLI recordings** — create stunning screenshots, videos, and GIFs of terminal interactions with ease. Perfect for documentation, tutorials, and showcasing command-line tools.

## ✨ Features

- 🎭 **Script-based automation** - Write terminal interactions as YAML scripts
- 📸 **High-quality screenshots** - Beautiful PNG captures with theming
- 🎬 **Smooth GIF recordings** - Create engaging animated demonstrations  
- 🚀 **Cross-platform** - Works on macOS, Linux, and Windows
- ⚡ **Fast & efficient** - Written in Rust for maximum performance
- 🎨 **Customizable themes** - Multiple built-in themes (Default, Dracula, etc.)
- 📜 **Simple YAML scripts** - Easy-to-write automation scripts

## 🚀 Quick Start

### Installation

```bash
# Install from source (requires Rust)
git clone https://github.com/kooshapari/KommandLineAutomation.git
cd KommandLineAutomation
cargo install --path .
```

### Basic Usage

```bash
# Take a screenshot of a single command
kla screenshot "ls -la" --output screenshot.png

# Record a script and generate a GIF
kla record examples/git-workflow.kla.yaml --output demo.gif --format gif

# Run an interactive demo
kla demo examples/git-workflow.kla.yaml --interactive
```

## 📝 Writing Scripts

KLA uses simple YAML scripts to define terminal automation sequences:

```yaml
name: "Git Workflow Demo"
settings:
  width: 120
  height: 30
  shell: "zsh"
  theme: "dracula"

steps:
  - type: command
    text: "git init"
    wait: "1s"
    
  - type: type
    text: "echo '# My Project' > README.md"
    speed: "50ms"
    
  - type: screenshot
    name: "readme-created"
    
  - type: command
    text: "git add README.md"
    
  - type: command
    text: "git commit -m 'Initial commit'"
    wait_for: "create mode"
    
  - type: record_gif
    duration: "5s" 
    name: "git-workflow"
```

### Script Elements

#### Settings
- `width` / `height`: Terminal dimensions
- `shell`: Shell to use (bash, zsh, fish, etc.)
- `theme`: Color theme (default, dracula)
- `working_dir`: Starting directory

#### Step Types
- `command`: Execute a shell command
- `type`: Type text with realistic speed
- `screenshot`: Capture a PNG screenshot
- `record_gif`: Record a GIF animation

#### Timing Control
- `wait`: Pause for a duration (`"1s"`, `"500ms"`)
- `speed`: Typing speed for realistic input
- `wait_for`: Wait for specific output text

## 🎨 Themes

### Built-in Themes

- **Default**: Clean, professional look
- **Dracula**: Popular dark theme with vibrant colors

### Custom Themes

```yaml
settings:
  theme: "dracula"  # or "default"
```

## 📚 Examples

### Simple Screenshot
```bash
kla screenshot "echo 'Hello, World!'" -o hello.png
```

### Git Workflow Demo
See [`examples/git-workflow.kla.yaml`](examples/git-workflow.kla.yaml) for a complete example showing:
- Repository initialization
- File creation and staging
- Commit workflow
- Screenshot and GIF generation

### Interactive Demo
```bash
kla demo examples/git-workflow.kla.yaml --interactive
```
Step through commands manually for live demonstrations.

## 🏗️ Architecture

KLA is built with a modular Rust architecture:

```
├── cli/          # Command-line interface
├── script/       # YAML script parsing and execution  
├── pty/          # Terminal control and automation
├── media/        # Screenshot and GIF generation
└── examples/     # Example scripts and demos
```

Key dependencies:
- `portable-pty` - Cross-platform terminal control
- `vt100` - Terminal emulation and parsing
- `image` & `gif` - Media generation
- `serde_yaml` - Script parsing

## 🛠️ Development

### Building from Source

```bash
git clone https://github.com/kooshapari/KommandLineAutomation.git
cd KommandLineAutomation
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes  
4. Add tests if applicable
5. Submit a pull request

## 📖 Use Cases

- **📚 Documentation** - Create visual guides for CLI tools
- **🎓 Tutorials** - Generate step-by-step terminal walkthroughs
- **🧪 Testing** - Automate TUI application testing  
- **🎬 Demos** - Showcase command-line tool functionality
- **🔄 CI/CD** - Generate automated demonstration assets

## 🤝 Inspiration

KLA draws inspiration from these excellent tools:
- [Playwright](https://playwright.dev) - Web automation framework
- [VHS](https://github.com/charmbracelet/vhs) - Write terminal GIFs as code
- [asciinema](https://asciinema.org) - Terminal session recording
- [t-rec](https://github.com/sassman/t-rec-rs) - Blazingly fast terminal recorder

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for amazing crates
- Terminal automation tool pioneers
- Contributors and users of KLA

---

Made with ❤️ in Rust