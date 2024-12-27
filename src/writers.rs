use std::fs::File;
use std::io::Write;

pub trait Writer {
    fn write(&mut self, string: &str);
    fn writeln(&mut self, string: &str, indent: i32);
}

pub struct StdWriter {
}

impl StdWriter {
    pub fn new() -> Self {
        StdWriter {}
    }
}

impl Writer for StdWriter {
    fn writeln(&mut self, string: &str, indent: i32) {
        print!("{:\t>indent$}{}\n", "", string, indent=indent as usize);
    }
    fn write(&mut self, string: &str) {
        print!("{}", string);
    }
}

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(file_name: &str) -> FileWriter {
        FileWriter {
            file: File::create(file_name).expect("Could not open file"),
        }
    }
}

impl Writer for FileWriter {
    fn writeln(&mut self, string: &str, indent: i32) {
        write!(self.file, "{:\t>indent$}{}\n", "", string, indent=indent as usize).expect("Could not write to file");
    }
    fn write(&mut self, string: &str) {
       write!(self.file, "{}", string).expect("Could not write to file");
    }
}

pub struct MultiWriter {
    writer_list: Vec<Box<dyn Writer>>,
}

impl MultiWriter {
    pub fn new() -> MultiWriter {
        MultiWriter {
            writer_list: Vec::new(),
        }
    }

    pub fn add_writer(&mut self, writer: Box<dyn Writer>) {
        self.writer_list.push(writer);
    }
}

impl Writer for MultiWriter {
    fn writeln(&mut self, string: &str, indent: i32) {
        for writer in self.writer_list.iter_mut() {
            writer.writeln(string, indent);
        }
    }
    fn write(&mut self, string: &str) {
        for writer in self.writer_list.iter_mut() {
            writer.write(string);
        }
    }
}