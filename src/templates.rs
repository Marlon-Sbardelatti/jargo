pub struct Templates;
use colored::*;

// store command name, alias and description as mod constants
const COMMAND_ALIASES_DESCS: [(&str, &str, &str); 5] = [
    ("help", "h", "Lists all commands available"),
    ("new", "n", "Creates a new java project"),
    ("run", "r", "Compiles and run your java project"),
    (
        "jrun",
        "j",
        "Just run the lastest compiled version of your project",
    ),
    ("create", "c", "Creates a new class in your java project"),
];

impl Templates {
    pub fn generate_main() -> String {
        r#"public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}"#
        .to_string()
    }

    pub fn generate_class(classname: &String) -> String {
        let class = format!(
            r#"public class {} {{

    
}}"#,
            classname
        );
        class
    }
    pub fn help() {
        println!("Java's package manager\n");
        println!("{} {}", "Usage:".green().bold(), "jargo [command]\n".cyan());
        println!("{}", "Commands:".green().bold());
        for (cmd, alias, desc) in COMMAND_ALIASES_DESCS {
            println!(
                "   {}, {}{}{}",
                cmd.cyan(),
                alias.cyan(),
                " ".repeat(10 - cmd.len() - alias.len()),
                desc
            );
        }
    }
}
