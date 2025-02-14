pub trait ToFile {
    fn save(&self, path: &str) -> std::io::Result<()>;

    fn load(&mut self, path: &str) -> std::io::Result<()>;
}