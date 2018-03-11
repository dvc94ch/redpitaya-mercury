extern crate redpitaya_mercury;

use redpitaya_mercury::prelude::*;
use redpitaya_mercury::la;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

// Max Ethernet frame size: 1500 bytes
// Max TCP payload: 1460 bytes (- 20 bytes IP header - 20 bytes TCP header)
// Max Sample rate: 125MSps
// LA buffer size: 16384 bytes
//
// Decimation factor of 15260:
// Sample rate: 125MSps / 10000 ~ 12.5kSps
// Buffer fills ~1/sec
// Takes 6 tcp packets to send a buffer

enum CommandKind {
    HwId = 0,
    ConfigGet = 1,
    ConfigSet = 2,
    AcquisitionStart = 3,
    AcquisitionStop = 4,
    Unknown,
    Error,
}

impl CommandKind {
    fn from(value: u8) -> Self {
        match value {
            0 => CommandKind::HwId,
            1 => CommandKind::ConfigGet,
            2 => CommandKind::ConfigSet,
            3 => CommandKind::AcquisitionStart,
            4 => CommandKind::AcquisitionStop,
            _ => CommandKind::Unknown,
        }
    }
}

struct Command {
    kind: CommandKind,
}

impl Command {
    fn from_stream(stream: &mut TcpStream) -> Self {
        // Accept u8 or *IDN? (for sigrok compat)
        let mut result = [0; 5];
        if stream.read(&mut result).is_err() {
            return Command { kind: CommandKind::Error };
        }
        if result[0] == '*' as u8 {
            return Command { kind: CommandKind::HwId };
        }
        let kind = CommandKind::from(result[0]);
        Command { kind }
    }
}

fn logic_analyzer() -> LogicAnalyzer {
    let mut la = LogicAnalyzer::new();
    la.default();
    la.set_input_mask(0xffff);
    la.set_input_polarity(0xffff);
    la.set_trigger_pre(la::BUFFER_SIZE / 2);
    la.set_trigger_post(la::BUFFER_SIZE / 2);
    la.set_decimation(10000);
    la.set_sync_source(SyncSource::La);
    la.set_trigger_source(TriggerSource::La);
    la
}

fn la_write_buffer(la: &mut LogicAnalyzer, stream: &mut TcpStream) {
    let mut tcp_buf = [0u8; la::BUFFER_SIZE as usize];

    loop {
        // reset and start
        la.reset();
        la.start_trigger();

        // reset and start
        la.reset();
        la.start_trigger();

        // wait for data
        while la.is_running() {}

        println!("Sending buffer");
        // print data
        let mut len = 0;
        for (i, sample) in la.buffer().iter().enumerate() {
            tcp_buf[i] = sample.read() as u8;
            len = i;
        }
        if stream.write(&tcp_buf[0..len]).is_err() {
            println!("Connection closed");
            return;
        } else {
            println!("Sent {} bytes", len);
        }
    }
}

fn get_hwid() -> String {
    // Vendor;Model;Firmware;SerialNumber
    let mut file = File::open("/sys/class/net/eth0/address").unwrap();
    let mut contents = "RedPitaya,RedPitaya,Mercury,".to_owned();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() {
    let hwid = get_hwid();
    println!("{}", hwid);

    let listener = TcpListener::bind("0.0.0.0:5555").unwrap();
    let mut la = logic_analyzer();

    println!("Listening on 0.0.0.0:5555");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted connection");
                let cmd = Command::from_stream(&mut stream);
                match cmd.kind {
                    CommandKind::HwId => {
                        println!("Received HwId");
                        if stream.write(&hwid.as_bytes()).is_err() {
                            println!("Error sending HwId");
                        }
                    },
                    CommandKind::ConfigGet => {
                        println!("Received ConfigGet");
                    },
                    CommandKind::ConfigSet => {
                        println!("Received ConfigSet");
                    },
                    CommandKind::AcquisitionStart => {
                        println!("Received AcquisitionStart");
                        la_write_buffer(&mut la, &mut stream);
                    },
                    CommandKind::AcquisitionStop => {
                        println!("Received AcquisitionStop");
                    },
                    CommandKind::Unknown => {
                        println!("Unknown command");
                    },
                    CommandKind::Error => {
                        println!("Error reading command");
                    }
                }
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
