pub trait DiskStorage {
    fn deserialize <R> (&mut self, src: R) -> Result<(), std::io::Error>
        where R: std::io::Read;
    fn serialize <W> (&self, dst: W) -> Result<(), std::io::Error>
        where W: std::io::Write;
}