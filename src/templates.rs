pub struct Templates;
use colored::*;

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
        println!("{} {}","Usage:".green().bold(), "jargo [command]\n".cyan());
        println!("{}", "Commands:".green().bold());
        println!("   {}      {}", "help, h".cyan(), "Lists all commands available");
        println!("   {}       {}", "new, n".cyan(), "Creates a new java project");
        println!("   {}       {}", "run, r".cyan(), "Compiles and run your java project");
        println!("   {}      {}", "jrun, j".cyan(), "Just run the lastest compiled version of your project");
        println!("   {}    {}", "create, c".cyan(), "Creates a new class in your java project");
    }
}
