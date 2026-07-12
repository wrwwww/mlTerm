use std::{
    io::{Read, Write},
    net::TcpStream,
};

use ssh2::{Channel, Session};

use crate::protocol::connection::TerminalSession;

mod client;
pub struct SshSession {
    session: Session,
    channel: Channel,
}
// impl SshSession {
//     pub fn new(addr: &str, port: u16) -> Self {
//         // Implementation for creating SSH session
//         let tcp = TcpStream::connect("192.168.1.100:22").unwrap();
//         let mut session = Session::new().unwrap();

//         session.set_tcp_stream(tcp);
//         Self {
//             session,
//             // channel: (),
//         }
//     }
// }
// impl TerminalSession for SshSession {
//     pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
//         self.channel.write_all(data)?;
//         self.channel.flush()?;
//         Ok(())
//     }

//     pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.channel.read(buf)
//     }

//     pub fn resize(&mut self, cols: u32, rows: u32) -> Result<(), ssh2::Error> {
//         self.channel.request_pty_size(cols, rows, None, None)
//     }

//     pub fn close(&mut self) -> Result<(), ssh2::Error> {
//         self.channel.close()
//     }
// }
