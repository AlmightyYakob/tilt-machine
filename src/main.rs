use serial::{self, unix::TTYPort, PortSettings, SerialPort};
use std::io::Read;

const FLAT: u8 = 48;
const VERTICAL: u8 = 49;

// How many times in a row the arduino has to register
// a value before considering it consistent
const TILT_THRESHOLD: i32 = 500;

struct TiltReader {
    buf: [u8; 1],
    port: TTYPort,
}
impl TiltReader {
    fn new() -> Result<TiltReader, std::io::Error> {
        let mut port = serial::open("/dev/ttyACM0")?;
        port.configure(&PortSettings {
            baud_rate: serial::Baud9600,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        })?;

        Ok(TiltReader { buf: [0; 1], port })
    }

    fn determine_position(&mut self) -> u8 {
        let mut tilt_length = 0;
        let mut current_orientation = FLAT;

        while tilt_length < TILT_THRESHOLD {
            if let Ok(_) = self.port.read_exact(&mut self.buf) {
                if self.buf[0] == current_orientation {
                    tilt_length += 1;
                } else {
                    current_orientation = self.buf[0];
                    tilt_length = 0;
                }
            }
        }

        return current_orientation;
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut reader = TiltReader::new()?;

    loop {
        let pos = reader.determine_position();
        println!("{}", pos);
    }
}
