use std::{io::{stdin, stdout, Write}, thread::sleep, time::Duration};

use colored::Colorize;
use serialport::{available_ports, SerialPort, SerialPortInfo};

use crate::compiling::error_handler;

#[derive(Copy, Clone)]
enum ArduinoCommand {
    Erase = 0,
    Write = 1,
    Ready = 2,
    Verify = 3
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
    write_command(&mut port, ArduinoCommand::Write);
    for b in &program {
        port.write(&[*b]).expect("Write failed!");
        sleep(Duration::from_millis(5));
    }
    println!("{}", "Finished upload".black());
    
    wait_until_ready(&mut port);
    println!("{}", "Verifying program".black());
    write_command(&mut port, ArduinoCommand::Verify);
    let mut current_address = 0;
    'main_loop: loop {
        let available_bytes = port.bytes_to_read().expect("Failed to read buff size");
        if available_bytes > 0 {
            let mut buffer: Vec<u8> = vec![0; available_bytes as usize];
            port.read(buffer.as_mut_slice()).expect("Found no data!");
            for b in buffer {
                if program[current_address] != b {
                    error_handler::print_error(&format!("Verification failed! Expected {:#x} Found {:#x} at {:#x}", program[current_address], b, current_address));
                    return;
                }

                current_address += 1;

                if current_address == program.len() {
                    break 'main_loop;
                }
            }
        } else {
            sleep(Duration::from_millis(10));
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
                }
            }
        }
        sleep(Duration::from_millis(10));
    }
}