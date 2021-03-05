use serial::{self, PortSettings, SerialPort};
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut port = serial::open("/dev/ttyACM0")?;
    // port.reconfigure(&|settings| {
    //     settings.borrow().set
    // });

    port.configure(&PortSettings {
        baud_rate: serial::Baud9600,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    })?;

    let mut buf = [0; 1];
    loop {
        if let Ok(_) = port.read_exact(&mut buf) {
            println!("{}", buf[0]);
        }
    }

    // Ok(())
}
