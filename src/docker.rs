// RustyNet docker command manager wrapper.


use std::error::Error;
use std::process::{Command, Stdio};
use std::fmt;

use log::{error, debug};

use crate::topology::{Link, Node};


// Interface for managing networking and docker containers.
pub struct DockerManager;

impl DockerManager {

    // Pulls all required images from registry.
    #[allow(dead_code)]
    pub fn pull_images() -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    // Create a link (by adding a network bridge on host), and connect its two
    // endpoints. Endpoints must have been running.
    pub fn create_link(link: &Link) -> Result<(), Box<dyn Error>> {
        debug!("Adding link '{}'...", link);
        run_command(true, "docker", &["network", "create", &link.name])?;
        run_command(true, "docker", &["network", "connect",
                                      &link.name, &link.ends.0])?;
        run_command(true, "docker", &["network", "connect",
                                      &link.name, &link.ends.1])?;
        Ok(())
    }

    // Destroy a link.
    pub fn destroy_link(link: &Link) -> Result<(), Box<dyn Error>> {
        debug!("Deleting link '{}'...", link);
        run_command(true, "docker", &["network", "disconnect",
                                      &link.name, &link.ends.0])?;
        run_command(true, "docker", &["network", "disconnect",
                                      &link.name, &link.ends.1])?;
        run_command(true, "docker", &["network", "rm", &link.name])?;
        Ok(())
    }

    // Create a node, i.e., run a docker container.
    pub fn create_node(node: &Node) -> Result<(), Box<dyn Error>> {
        debug!("Running node '{}'...", node);
        run_command(true, "docker", &["run", "-t", "-d", "--privileged",
                                      "--name", &node.name,
                                      "rustynet/node"])?;
        Ok(())
    }

    // Stop and remove a node's container.
    pub fn destroy_node(node: &Node) -> Result<(), Box<dyn Error>> {
        debug!("Removing node '{}'...", node);
        run_command(true, "docker", &["stop", &node.name])?;
        run_command(true, "docker", &["rm", &node.name])?;
        Ok(())
    }
}


// Helper function to run a command.
fn run_command(privileged: bool, cmd: &str,
               args: &[&str]) -> Result<(), Box<dyn Error>> {
    if privileged {
        if Command::new("sudo")
                   .stdout(Stdio::null())
                   .arg("--")
                   .arg(cmd)
                   .args(args)
                   .status()
                   .is_err() {
            error!("Failed to execute command '{}' (as root).", cmd);
            Err(Box::new(CommandExecError))
        } else {
            Ok(())
        }
    } else {
        if Command::new(cmd)
                   .stdout(Stdio::null())
                   .args(args)
                   .status()
                   .is_err() {
            error!("Failed to execute command '{}' (as user).", cmd);
            Err(Box::new(CommandExecError))
        } else {
            Ok(())
        }
    }
}


// Command execution error type.
#[derive(Debug, Clone)]
struct CommandExecError;

impl fmt::Display for CommandExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something wrong when executing a shell command")
    }
}

impl Error for CommandExecError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
