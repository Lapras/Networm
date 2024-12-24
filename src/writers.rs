pub trait Writer {
    fn write(&self, string: String);
    fn writeln(&self, string: String, indent: i32);
}

pub struct StdWriter {
}

impl StdWriter {
    pub fn new() -> Self {
        StdWriter {}
    }
}

impl Writer for StdWriter {
    fn writeln(&self, string: String, indent: i32) {
        print!("{:\t>indent$}{}\n", "", string, indent=indent as usize);
    }
    fn write(&self, string: String) {
        print!("{}", string);
    }
}