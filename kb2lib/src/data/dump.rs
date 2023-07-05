pub trait Dump {
    fn dump(&self) -> String;
}

impl Dump for (u64, u64) {
    fn dump(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}