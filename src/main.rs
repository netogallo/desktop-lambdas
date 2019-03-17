mod parser;

fn main() {
    let res = parser::parse(&String::from("/usr/share/applications/calligra.desktop"));
    println!("Hello, world!");
}
