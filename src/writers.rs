pub trait Writer {
    fn write(&self, string: String);
}

pub struct StdWriter {
}

impl StdWriter {
    pub fn new() -> Self {
        StdWriter {}
    }
}

impl Writer for StdWriter {
    fn write(&self, string: String) {
        print!("{}", string);
    }
}