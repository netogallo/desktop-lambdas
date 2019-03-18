use std::iter::*;
use std::io::prelude::*;

use regex::Regex;

pub struct Entry{
    key: String,
    value: String
}

impl Entry{
    pub fn create(key: String, value: String) -> Entry{
        return Entry{key: key, value: value};
    }

    pub fn try_parse_entry(entry: &String) -> Option<Entry> {
        let kv = Vec::from_iter(entry.split("="));

        if kv.len() == 2 {
            return Some(
                Entry::create(
                    String::from(kv[0]),
                    String::from(kv[1])));
        }

        return None;
    }
}

pub struct Section{
    name: String,
    entries: Vec<Entry>
}

impl Section{

    pub fn from_header(s: String) -> Section{
        return Section{
            name: s,
            entries: Vec::new()
        }
    }

    pub fn try_parse_header(s: String) -> Option<Section>{
        let re = Regex::new(r"[((\w|\s)+)]").unwrap();
        let captures = re.captures_iter(&s);

        for cap in captures{
            let s : String = String::from(cap.get(0).unwrap().as_str());
            let r = Section::from_header(s);
            return Option::Some(r);
        }

        return Option::None;
    }

    pub fn add_entries(&mut self, mut entries: Vec<Entry>){
        self.entries.append(entries.as_mut());
    }
}