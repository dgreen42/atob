use ::std::{
    env::args,
    fs::{read, File},
    io::{Error, Write},
};
use std::{
    error,
    io::{stdout, BufRead},
};

fn main() {
    use conversion::*;
    let file = args().nth(1).expect("Enter file");
    let option = args().nth(2).expect("Enter option");
    let mut file_name = file.split(".").next().unwrap().to_string();
    let ext = file.split(".").last().unwrap();
    let old_delim = get_ext_old(ext);
    let new_ext = get_ext_new(&option);

    let reader = read(file).expect("could not read file");
    file_name.push_str(&new_ext);
    let mut writer = File::create(file_name).expect("Could not create new file");

    for line in reader.lines() {
        let line_info = Line {
            old_delim: old_delim.clone(),
            line: line.unwrap(),
            option: option.clone(),
        };
        let new_line = line_info.convert();
        let result = writer.write_all(new_line.as_bytes());
        match result.unwrap() {
            () => continue,
            Error => stdout().write_all(b"Bad write\n").unwrap(),
        }
    }
}

mod conversion {
    use std::io::{stdout, Write};
    pub struct Line {
        pub old_delim: String,
        pub line: String,
        pub option: String,
    }

    impl Line {
        pub fn convert(&self) -> String {
            let mut new_line = String::new();
            let mut new_delim = String::new();
            if self.option == "-c" {
                new_delim.push_str(",");
            }
            if self.option == "-t" {
                new_delim.push_str("\t");
            }
            for ch in self.line.chars() {
                if ch.to_string() != self.old_delim {
                    new_line.push_str(&ch.to_string());
                } else {
                    new_line.push_str(&new_delim);
                }
            }
            new_line.push_str("\n");
            new_line
        }
    }

    pub fn get_ext_new(extention: &str) -> String {
        let mut delim = String::new();
        match extention {
            "-c" => delim.push_str(".csv"),
            "-t" => delim.push_str(".tsv"),
            _ => stdout()
                .write_all(b"Extension has not been implemented\n")
                .unwrap(),
        }
        delim
    }

    pub fn get_ext_old(extention: &str) -> String {
        let mut delim = String::new();
        match extention {
            "csv" => delim.push_str(","),
            "tsv" | "bed" => delim.push_str("\t"),
            _ => stdout()
                .write_all(b"Extension has not been impelented\n")
                .unwrap(),
        }
        delim
    }
}
