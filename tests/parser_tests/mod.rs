extern crate desktop_lambdas;

#[cfg(test)]
mod parser_tests {

    use desktop_lambdas::parser;

    static ENTRY: &'static str = "
        [Desktop Entry]\n
        Type=Application\n
        Name=Calligra\n
        Name[ast]=Calligra\n
        Name[bg]=Calligra
    ";

    #[test]
    fn it_parses_file() {
        //assert_eq!(2 + 2, 2);

        let input = String::from(ENTRY);
        let res = parser::parse_str(&input);

        match res{
            Ok(sections) => {
                for section in sections{
                    println!("{}{{", section.name);
                    assert_eq!(section.name, "Desktop Entry");

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
    }
}