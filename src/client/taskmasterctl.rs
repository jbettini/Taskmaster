/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterctl.rs                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:19:09 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/26 06:09:11 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod command;
use command::Command;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

fn print_help() {
    println!(
        "Commands accepted:
        - help => print all the accepted commands
        - status => show programs status
        - reload => reload the main program
        - start \"program name\" => starting a program
        - restart \"program name\" => restarting a program
        - stop \"program name\" => stopping a program
        - stop \"daemon\" => stopping the main program\n
        - Use exit or ctrl + D to Quit the program"
    );
}

pub fn taskmasterctl() {

    // #set readline
    let mut rl = DefaultEditor::new().expect("Error: Init ReadLine");
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    // #set socket
    let socket_path = "/Users/xtem/test/unixServer/sock/mysocket.sock";
    let mut us =
        UnixStream::connect(socket_path).expect("Could not create stream");
    loop {
        let read_line = rl.readline("$$> ");
        match read_line {
            Ok(line) => { 
                let _ = rl.add_history_entry(line.as_str());
                if let Some(mut command) = Command::parse(line) {
                    match command.cmd.as_str() {
                        "start" => command.handle_cmd(&us),
                        "stop" => command.handle_cmd(&us),
                        "restart" => command.handle_cmd(&us),
                        "status" => command.handle_cmd(&us),
                        "reload" => command.handle_cmd(&us),
                        "help" => print_help(),
                        "exit" => {
                            println!("Quit");
                            break;
                        },
                        _ => panic!("Unknown command: Parsing error"),
                    }
                } else {
                    println!("Invalid input : Use help to show all commands");
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // let _ = rl.save_history("history.txt");
}