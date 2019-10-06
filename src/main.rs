// RustyNet CLI tool.


mod logger;
mod topology;
mod docker;
mod shell;

use nix::unistd::getuid;
use log::{error, info, debug, LevelFilter};

use logger::ColoredLogger;
use topology::Topology;
use shell::RustyShell;


// Main entrance of the RustyNet CLI tool.
fn main() {

    // Set logging level - Debug, Info, or Error.
    ColoredLogger::init(LevelFilter::Debug);

    // Only supports Linux, because docker containers require a Linux host
    // as the host machine. Mac / Win can have Docker Machine, but currently
    // not supported.
    debug!("Checking OS platform...");
    if cfg!(target_os = "linux") == false {
        error!("Currently, RustyNet only supports Linux.");
        panic!("Platform error.");
    }

    // Check if run as root user.
    debug!("Checking root user privilege...");
    if !getuid().is_root() {
        error!("RustyNet should be run as root privilege.");
        panic!("Priviledge error.");
    }

    // Build up the topology of the "minimal" example in topolib.
    debug!("Reading topology from YAML config file...");
    let config_file = "topolib/minimal.yml";
    let topo = Topology::new(Some(config_file.to_owned()))
                   .unwrap_or_else(|err| {
        error!("Reading config file '{}' failed. Error: {}.",
               config_file, err);
        panic!("Config file error.");
    });
    info!("Initial topology generated: {}", &topo);

    // Deploy the topology.
    if topo.deploy().is_err() {
        error!("Topology deployment failed.");
        topo.clean().unwrap();
        panic!("Deployment error.")
    }
    info!("Topology deployment succeeds, congrats ;)");

    // Open up an interactive RustyNet shell session, to do whatever
    // emulation tasks you want.
    RustyShell::invoke(&topo);

    // Clean up the deployment.
    if topo.clean().is_err() {
        error!("Topology clean up failed.");
        panic!("Clean up error.")
    }
    info!("Topology cleaned up.");
}
