pub struct Templates;

impl Templates {
    pub fn generate_main() -> String {
        r#"public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}"#
        .to_string()
    }
}