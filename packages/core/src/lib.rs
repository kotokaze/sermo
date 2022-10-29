use serialport::{self, new, SerialPort};
use std::io;
use std::time::Duration;

pub fn open_port(
  port: std::borrow::Cow<'_, str>,
  baud: u32,
) -> Result<Box<dyn SerialPort>, serialport::Error> {
  let port = new(port, baud)
    .timeout(Duration::from_millis(10u64))
    .open()?;
  Ok(port)
}

pub fn print_serial(port: &mut Box<dyn SerialPort>) -> Result<(), io::Error> {
  let mut buf = [0u8; 1024];
  loop {
    match port.read(buf.as_mut_slice()) {
      | Ok(t) => {
        io::Write::write(&mut io::stdout(), &buf[..t])?;
      },
      | Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
      | Err(e) => eprintln!("{:?}", e),
    }
  }
}
