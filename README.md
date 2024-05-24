# Jargo

## Description
Jargo is a Rust-based tool that functions similarly to Cargo but is designed for Java projects. It simplifies the process of managing Java projects by providing commands to create, compile, and run your Java code efficiently.

## Demo
https://github.com/Marlon-Sbardelatti/jargo/assets/117592329/07cea649-326e-4d9a-86fc-ed69bc9ce5ca


## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation
To install Jargo, follow these steps:

1. Clone the repository:
```bash
git clone https://github.com/yourusername/jargo.git
cd jargo
```
2. Build the project using Cargo (Rust's package manager):
```bash
cargo build --release
```
3. Copy the Jargo executable to /usr/local/bin:
```bash
sudo cp target/release/jargo /usr/local/bin
```
## Usage
To use Jargo, navigate to your desired directory and run one of the following commands:

## Creating a New Java Project
```bash
jargo new myproject
# or
jargo n myproject
```
## Running Your Project
```bash
jargo run
# or
jargo r
```
## Running the Latest Compiled Version
```bash
jargo jrun
# or
jargo j
```
## Creating a New Java Class
```bash
jargo create MyClass
# or
jargo c MyClass
```
## Listing All Commands
```bash
jargo help
# or
jargo h
```
## Commands
* new or n: Creates a new Java project.
* run or r: Compiles and runs the project.
* jrun or j: Runs the latest compiled version of the project.
* help or h: Lists all commands.
* create or c: Creates a new Java class in the src directory.

## Project Structure
When you create a new project with Jargo, the following structure is set up:

```css
myproject/
├── src/
│   └── Main.java
└── out/
```
* src: Contains your Java source files.
* out: The destination for your compiled classes.
  
As of now, you can only place files inside the src directory. You can run or jrun commands from any directory within your project.

## Contributing
We welcome contributions! Follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (git checkout -b feature/awesome-feature).
3. Commit your changes (git commit -m 'Add some awesome feature').
4. Push to the branch (git push origin feature/awesome-feature).
5. Open a pull request.

## License
This project is licensed under the MIT License.

## Contact
Feel free to reach out with any questions or feedback.

* Email: hetzwga@gmail.com
* GitHub: Marlon-Sbardelatti
