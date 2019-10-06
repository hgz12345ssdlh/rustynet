// RustyNet CLI shell implementation.


use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};

use log::{error, info};
use colored::*;

use crate::topology::Topology;


// RustyNet shell wrapper.
pub struct RustyShell;

impl RustyShell {

    // Invoke the interactive shell, loops until 'exit' command.
    pub fn invoke(topo: &Topology) {
        loop {

            // Show prompt.
            RustyShell::show_prompt();
            if stdout().flush().is_err() {
                continue;
            }

            // Read in user command into 'input', split into piped commands.
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut cmds = input.trim().split(" | ").peekable();
            let mut prev_cmd = None;

            // Handle one by one.
            while let Some(cmd) = cmds.next()  {

                let mut parts = cmd.trim().split_whitespace();
                let cmd = parts.next().unwrap();
                let args = parts;

                match cmd {

                    // Change working directory unsupported.
                    "cd" => error!("Change working directory not supported."),

                    // If given 'exit', end this interactive session.
                    "exit" => return,

                    // Other commands sent directly to the system shell.
                    _ => {

                        // If command starts with a host's name, intepret it
                        // as `docker exec -it ...` so that it redirects to
                        // be executed on that host.
                        let mut command = cmd;
                        let mut preargs: Vec<&str> = vec![];
                        if let Some(_) = topo.hosts.iter()
                                             .find(|&n| n.name == cmd) {
                            command = "docker";
                            preargs.extend(&["exec", "-it", cmd]);
                            info!("Prefixed with 'docker exec -it'.");
                        }

                        // Redirect input from previous command if needed.
                        let stdin = prev_cmd
                            .map_or(
                                Stdio::inherit(),
                                |output: Child| {
                                    Stdio::from(output.stdout.unwrap())
                                }
                            );

                        // There is another command piped behind this one,
                        // then prepare to send output to the next command.
                        let stdout = if cmds.peek().is_some() {
                            Stdio::piped()
                        } else {    // No more commands in pipe.
                            Stdio::inherit()
                        };

                        let output = Command::new(&command)
                                             .args(preargs)
                                             .args(args)
                                             .stdin(stdin)
                                             .stdout(stdout)
                                             .spawn();

                        match output {
                            Ok(output) => prev_cmd = Some(output),
                            Err(err) => {
                                prev_cmd = None;
                                error!("{}", err);
                            },
                        };
                    }
                }
            }

            // Wait for the final command to finish.
            if let Some(mut final_command) = prev_cmd {
                if final_command.wait().is_err() {
                    continue;
                }
            }
        }
    }

    // Helper function to show the shell prompt.
    fn show_prompt() {
        print!("{} ", "RustyNet>".purple().bold().to_string());
    }
}
