extern crate desktop_lambdas;
use desktop_lambdas::parser;

fn main() {
    let res = parser::parse(&String::from("/usr/share/applications/calligra.desktop"));

    match res{
        Ok(sections) => {
            for section in sections{
                println!("{}{{", section.name);

                for entry in section.entries{
                    println!("\t{}:{},", entry.key, entry.value);
                }

                println!("}}");
            }
        }
        Err(error) => {
            println!("{}", error.to_string());
            println!("{}", error.to_string());
        }
    }
    println!("Hello, world!");
}