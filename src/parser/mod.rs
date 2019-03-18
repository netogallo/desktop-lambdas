use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::iter::*;

mod desktop;
mod error;

use desktop::*;
use error::*;

struct EntryIterator<'a, 'x>{
    parser: &'a mut ParserState<'x>
}

impl<'a, 'x> Iterator for EntryIterator<'a, 'x>{
    type Item = Entry;

    fn next(&mut self) -> Option<Entry>{
        return self.parser.next_entry();
    }
}

struct ParserState<'a>{
    current_line: String,
    buffer: Box<Iterator<Item = &'a Result<String, std::io::Error>>>,
    empty: bool
}

impl<'a> ParserState<'a>{

    fn try_update_current(&mut self, value: &'a Result<String, std::io::Error>) -> bool{
        match value{
            Ok(line) => {
                self.current_line = line.clone();
                return true;
            }
            Err(_) => { return false; }
        }
    }

    fn advance(&mut self) -> bool{
        let result = match self.buffer.next(){
            Some(line) => { self.try_update_current(line) }
            None => { false }
        };

        self.empty = !result;
        return result;
    }

    pub fn next_entry(&mut self) -> Option<Entry>{
        if self.empty {
            return None;
        }
        
        let result = Entry::try_parse_entry(&self.current_line);
        if result.is_some() {
            self.advance();
        }

        return result;
    }

    pub fn next_entries(&mut self) -> Vec<Entry>{
        return Vec::from_iter(EntryIterator{parser: self});
    }

    pub fn next_section(&mut self) -> Option<Section>{

        if self.empty {
            return None;
        }

        match Section::try_parse_header(self.current_line.clone()) {
            Some(header) => {
                let mut result = header;
                self.advance();
                result.add_entries(self.next_entries());

                return Some(result);
            }
            None => { return None; }
        }
    }

    fn empty() -> ParserState<'a>{
        let mut vc : Vec<Result<String, std::io::Error>> = Vec::new();
        let mut vit = vc.iter();
        let mut it = Box::from(Vec::new().iter());

        return ParserState {
            current_line: String::from(""),
            buffer: it,
            empty: true
        }
    }

    pub fn new(mut it : Box<Iterator<Item = &'a Result<String, std::io::Error>>>) -> ParserState<'a>{

        return match it.next() {
            Some(initial) => {
                match initial {
                    Ok(initial) => {
                        ParserState {
                            current_line: initial.clone(),
                            buffer: it,
                            empty: false
                        }
                    }
                    Err(_) => {
                        ParserState::empty()
                    }
                }
            }
            None => { ParserState::empty() }
        }
    }
}

impl<'a> Iterator for ParserState<'a>{
    type Item = Section;

    fn next(&mut self) -> Option<Section>{
        return self.next_section();
    }
}

pub fn parse_entry(line: String) -> Result<Entry, ParseError>{
    let kv = Vec::from_iter(line.split("="));

    if kv.len() != 2 {
        let message = [
            String::from("The line '"),
            line,
            String::from("' is not valid.")]
            .concat();
        return Err(ParseError::create(message));
    }

    return Ok(Entry::create(
        String::from(kv[0]),
        String::from(kv[1])));
}

fn parse_line_input(line: Result<String, std::io::Error>) -> Result<Entry, ParseError>{
    match line{
        Ok(_line) => { return parse_entry(_line); }
        Err(error) => { return Err(ParseError::from_error(error)); }
    }
}

fn parse_section<I : Iterator<Item = Result<String, std::io::Error>>>(lines: &mut I) -> Option<Section>{
    return Option::None;
}

pub fn parse_sections<I : Iterator<Item = Result<String, std::io::Error>>>(mut lines: I) -> Vec<Section>{
    let mut result = Vec::new();
    let mut section = parse_section(&mut lines);

    while section.is_some() {
        result.push(section.unwrap());
        section = parse_section(&mut lines);
    }

    return result;
}

pub fn parse(location: &String) -> Result<Vec<Entry>, ParseError> {

    let file = File::open(location);

    match file{
        Ok(content) => {
            let buff = BufReader::new(content);
            parse_sections(buff.lines());
            let mut acc : Vec<Entry> = Vec::new();

            return Ok(acc);
        }
        Err(error) => {
            return Err(
                ParseError::from_error(error));
        }
    }
}