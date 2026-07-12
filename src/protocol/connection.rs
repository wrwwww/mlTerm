pub trait TerminalSession: Send {
    fn write(&mut self, data: &[u8]);

    fn resize(&mut self, cols: u16, rows: u16);

    fn close(&mut self);

    fn read(&mut self) -> Option<Vec<u8>>;
}