use std::{env, process};
use std::{thread, time::Duration, str};

fn main() {

    let mut serial_port: String = String::new();
    let mut tenma_cmd: String = String::new();

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => usage(),
            "--on" => tenma_cmd = "OUT1".to_string(),
            "--off" => tenma_cmd = "OUT0".to_string(),
            "--get-ident" => tenma_cmd = "*IDN?".to_string(),
            "--get-max-voltage" => tenma_cmd = "VSET1?".to_string(),
            "--get-voltage" => tenma_cmd = "VOUT1?".to_string(),
            "--set-voltage" => {
                if let Ok(v_value) = args.next().expect("No value given for voltage!")
                    .trim().parse::<f32>() {

                    tenma_cmd = "VSET1:".to_string();
                    tenma_cmd.push_str(&v_value.to_string());
                    tenma_cmd.push_str("\r\n");

                } else {
                    panic!("Invalid value for voltage!");
                }
            }
            "--get-max-current" => tenma_cmd = "ISET1?".to_string(),
            "--get-current" => tenma_cmd = "IOUT1?".to_string(),
            "--set-current" => {
                if let Ok(c_value) = args.next().expect("No value given for current!")
                    .trim().parse::<f32>() {

                    tenma_cmd = "ISET1:".to_string();
                    tenma_cmd.push_str(&c_value.to_string());
                    tenma_cmd.push_str("\r\n");

                } else {
                    panic!("Invalid value for current!");
                }

            }
            _ => {
                if Some(0) == arg.find("/dev/tty") {
                    serial_port = arg;
                }
                else {
                    println!("Unkown argument {}", arg);
                    process::exit(1_i32);
                }
            }
        }
    } 

    println!("[DEBUG] cmd: {}", tenma_cmd);

    send_tenma_cmd(&serial_port, tenma_cmd.as_bytes());
}

fn send_tenma_cmd(serial_port: &str, cmd: &[u8]) {

    if serial_port.is_empty() {
        usage()
    }

    let mut tenma_port = serialport::new(serial_port, 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open serial port!");

    tenma_port.write(cmd).expect("Write failed!");

    thread::sleep(Duration::from_millis(1000));

    let mut serial_buf: Vec<u8> = vec![0; 32];
    if let Ok(_res) = tenma_port.read(serial_buf.as_mut_slice()) {
        println!("{}", str::from_utf8(&serial_buf).unwrap());
    }
}

fn usage() {
    println!("Usage: rust_tenma <serial port> [COMMANDS]");
    println!(" --on                     Turns Tenma Power Suply On");
    println!(" --off                    Turns Tenma Power Suply Off");
    println!(" --get-ident              Gets Tenma Identification");
    println!(" --get-max-voltage        Gets Maximum Voltage");
    println!(" --get-voltage            Gets Voltage Consumption");
    println!(" --set-voltage <value>    Sets Maximum Voltage");
    println!(" --get-max-current        Gets Maximum Current");
    println!(" --get-current            Gets Current Consumption");
    println!(" --set-current <value>    Sets Maximum Current");
    println!("\nExamples:");
    println!("  ./rust_tenma /dev/ttyACM0 --on");
    println!("  ./cargo run -- /dev/ttyACM0 --on");
    println!("  ./rust_tenma /dev/ttyACM0 --set-voltage 12");
    process::exit(1_i32);
}
