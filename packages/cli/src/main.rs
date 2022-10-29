mod cli;
use sermo_core::{open_port, print_serial};

fn main() {
  let cli = cli::parse();
  if !cli.device.exists() {
    println!("Device file does not exist");
    std::process::exit(-1i32);
  }

  let device = cli.device.into_os_string();
  println!("Selected device: {:?}", device);

  let mut port = open_port(device.to_string_lossy(), cli.baud).unwrap();
  println!("Opened port: {:?}", port.name());

  print_serial(&mut port).unwrap();
}
