use serial::{self, unix::TTYPort, PortSettings, SerialPort};
use std::{io::Read, process::Command};

const FLAT: u8 = 48;
const VERTICAL: u8 = 49;
const XRANR_MONITOR: &str = "HDMI-0";

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

fn change_orientation(orientation: u8) {
    let rotation = if orientation == VERTICAL {
        "left"
    } else {
        "normal"
    };

    Command::new("xrandr")
        .args(&["--output", XRANR_MONITOR, "--rotate", rotation])
        .output()
        .expect("Failed to execute process");
}

pub fn run() {
    let mut reader = TiltReader::new().expect("Couldn't open serial port reader");
    let mut orientation = FLAT;

    loop {
        let new_orientation = reader.determine_position();
        if new_orientation != orientation {
            orientation = new_orientation;
            change_orientation(orientation);
        }
    }
}
