use serialport;

fn main() {
  let ports = serialport::available_ports().expect("No ports found!");
  for (idx, port) in ports.iter().enumerate() {
    println!("{}: {}", idx, port.port_name);
  }
}
