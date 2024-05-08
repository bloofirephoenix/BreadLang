use std::{io::{stdin, stdout, Write}, thread::sleep, time::{Duration, SystemTime}};

use colored::Colorize;
use serialport::{available_ports, SerialPort, SerialPortInfo};

use crate::compiling::error_handler;

#[derive(Copy, Clone)]
enum ArduinoCommand {
    Erase = 0,
    Write = 1,
    Ready = 2,
    Verify = 3,
    Stop = 4
}

pub fn upload(program: Vec<u8>) {
    let mut devices: Vec<SerialPortInfo> = Vec::new();
    println!("Available Devices:");
    for p in available_ports().expect("No ports found") {
        println!("{}: {}", devices.len() + 1, p.port_name);
        devices.push(p);
    }

    print!("Select a Device: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let selection: usize;

    if let Ok(num) = buffer.trim().parse::<usize>() {
        selection = num;
    } else {
        error_handler::print_error("Invalid number");
        return;
    }

    let device: &SerialPortInfo;

    if let Some(d) = &devices.get(selection - 1) {
        device = d;
    } else {
        error_handler::print_error("That is not an option");
        return;
    }

    println!("{}", "Connecting".black());
    let mut port = serialport::new(&device.port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open().expect("Failed to open port");

    wait_until_ready(&mut port);
    println!("{}", "Erasing chip".black());
    write_command(&mut port, ArduinoCommand::Erase);
    wait_until_ready(&mut port);

    println!("{}", "Uploading program".black());

    let max_buffer_size = 60;
    let mut i = 0;
    while i < program.len() {
        let start = SystemTime::now();
        let buffer_size = usize::min(program.len() - i, max_buffer_size);
        let bytes = &program[i..i + buffer_size];
        
        // write the size of the buffer
        write_command(&mut port, ArduinoCommand::Write);
        wait_until_ready(&mut port);
        port.write(&[buffer_size as u8]).expect("Write failed!");

        wait_until_ready(&mut port);

        // write buffer
        port.write(bytes).expect("Write failed!");

        wait_until_ready(&mut port);

        i += buffer_size;

        if i % (max_buffer_size * 10) == 0 {
            let time = SystemTime::now().duration_since(start).unwrap().as_millis();
            let time = (time / buffer_size as u128) * (program.len() - i) as u128;
            let time = format_time(time);
            println!("{}", format!("Uploading... {}% ({} remaining)", ((i + 1) as f32 / program.len() as f32) * 100 as f32, time).black());
        }
    }

    println!("{}", "Finished upload".black());
    
    println!("{}", "Verifying program".black());
    write_command(&mut port, ArduinoCommand::Verify);
    let mut current_address = 0;
    
    let mut start = SystemTime::now();
    'main_loop: loop {
        let available_bytes = port.bytes_to_read().expect("Failed to read buff size");
        if available_bytes > 0 {
            let mut buffer: Vec<u8> = vec![0; available_bytes as usize];
            port.read(buffer.as_mut_slice()).expect("Found no data!");
            for b in buffer {
                if program[current_address] != b {
                    error_handler::print_error(&format!("Verification failed! Expected {} Found {} at {}", program[current_address], b, current_address));
                    return;
                }

                current_address += 1;

                if current_address == program.len() {
                    write_command(&mut port, ArduinoCommand::Stop);
                    break 'main_loop;
                }

                if current_address % 1000 == 0 {
                    let time = SystemTime::now().duration_since(start).unwrap().as_millis() as f64;
                    let time = (program.len() as f64 - current_address as f64) * (time / 1000.0);
                    let time = format_time(time.ceil() as u128);
                    println!("{}", format!("Verifying... {}% ({} remaining)", (current_address as f32 / program.len() as f32) * 100.0, time).black());
                    start = SystemTime::now();
                }
            }
        } else {
            sleep(Duration::from_millis(1));
        }
    }
    
    println!("{}", "Finished".green().bold());
}

fn write_command(port: &mut Box<dyn SerialPort>, command: ArduinoCommand) {
    port.write(&[command as u8]).expect("Write failed!");
}

fn wait_until_ready(port: &mut Box<dyn SerialPort>) {
    loop {
        let available_bytes = port.bytes_to_read().expect("Failed to read buff size");
        if available_bytes > 0 {
            let mut buffer: Vec<u8> = vec![0; available_bytes as usize];
            port.read(buffer.as_mut_slice()).expect("Found no data!");
            for b in buffer {
                if b == ArduinoCommand::Ready as u8 {
                    return;
                } else {
                    println!("Received weird byte {}", b);
                }
            }
        }
        sleep(Duration::from_millis(1));
    }
}

fn format_time(mut time: u128) -> String {
    let mut str = String::from("");

    // hours
    if time >= 3600000 {
        let hours = time / 3600000;
        time = time % 3600000;
        str += &format!("{}h ", hours);
    }
    // minutes
    if time >= 60000 {
        let minutes = time / 60000;
        time = time % 60000;
        str += &format!("{}m ", minutes);
    }

    // seconds
    if time >= 1000 {
        let seconds = time / 1000;
        time = time % 1000;
        str += &format!("{}s", seconds);
    }
    if !str.is_empty() {
        str.trim().to_string()
    } else {
        String::from("0s")
    }
}