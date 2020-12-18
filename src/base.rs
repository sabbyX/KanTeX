
pub trait Stringify: Send + Sync {
    fn stringify(&self) -> String;
}

impl Stringify for &str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}
