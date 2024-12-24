use std::fs::{self, File};
use std::io::BufWriter;
use std::io::Write;

pub trait Writer {
    fn write(&mut self, string: String);
    fn writeln(&mut self, string: String, indent: i32);
}

pub struct StdWriter {
}

impl StdWriter {
    pub fn new() -> Self {
        StdWriter {}
    }
}

impl Writer for StdWriter {
    fn writeln(&mut self, string: String, indent: i32) {
        print!("{:\t>indent$}{}\n", "", string, indent=indent as usize);
    }
    fn write(&mut self, string: String) {
        print!("{}", string);
    }
}

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(file_name: String) -> FileWriter {
        FileWriter {
            file: File::create(file_name).expect("Could not open file"),
        }
    }
}

impl Writer for FileWriter {
    fn writeln(&mut self, string: String, indent: i32) {
        write!(self.file, "{:\t>indent$}{}\n", "", string, indent=indent as usize).expect("Could not write to file");
    }
    fn write(&mut self, string: String) {
       write!(self.file, "{}", string).expect("Could not wwrite to file");
    }
}