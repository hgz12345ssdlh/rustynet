// RustyNet components & topology.


use std::fs::read_to_string;
use std::error::Error;
use std::fmt;

use tuple::tuple;
use log::{error, info, debug};
use yaml_rust::YamlLoader;

use crate::docker::DockerManager;


// Topology of the network.
#[derive(Debug)]
pub struct Topology {
    pub hosts: Vec<Node>,
    pub switches: Vec<Node>,
    pub links: Vec<Link>,
}

impl Topology {

    // Generate a new topology. If given a YAML file, initialize according
    // to that config file. If given None, produce an empty topology.
    pub fn new(config: Option<String>) -> Result<Topology, Box<dyn Error>> {

        // Given a config file, parse it to get initial topology.
        if let Some(file) = config {

            // Parse the YAML config file.
            info!("Scanning YAML config file: {}", file);
            let docs = YamlLoader::load_from_str(&read_to_string(file)?)?;
            let spec = &docs[0];

            // Parse nodes.
            let hosts = spec["hosts"].as_str()
                                     .unwrap_or("")
                                     .split_ascii_whitespace()
                                     .map(|name| { Node::new(name) })
                                     .collect::<Vec<Node>>();
            // TODO: check duplicate hosts.
            debug!("Hosts found: {:?}", &hosts);

            // Parse nodes.
            let switches = spec["switches"].as_str()
                                           .unwrap_or("")
                                           .split_ascii_whitespace()
                                           .map(|name| { Node::new(name) })
                                           .collect::<Vec<Node>>();
            // TODO: check duplicate switches.
            debug!("switches found: {:?}", &switches);

            // Create topology instance.
            let mut topo = Topology { hosts, switches, links: vec![] };

            // Parse links.
            for link in spec["links"].as_vec().unwrap() {

                // Split node names field.
                let ends = link.as_str()
                               .unwrap_or("")
                               .split_ascii_whitespace();
                if ends.clone().count() != 2 {
                    error!("Each link must have exactly two endpoints.");
                    return Err(Box::new(ConfigParsingError));
                }
                let ends: (&str, &str) = tuple(ends).unwrap();

                // Find ref to nodes of that names.
                if topo.node_by_name(ends.0).is_none() ||
                   topo.node_by_name(ends.1).is_none() {
                    error!("Some host listed in 'ends' field not fould.");
                    return Err(Box::new(ConfigParsingError));
                }
                topo.links.push(Link::new(ends));
                debug!("Link added: {:?}", &topo.links.last().unwrap());
            }
            // TODO: comprehensive error condition checking.

            Ok(topo)

        // No file given, return empty topology.
        } else {
            let topo = Topology {
                hosts: vec![],
                switches: vec![],
                links: vec![],
            };
            Ok(topo)
        }
    }

    // Find a node by name.
    pub fn node_by_name(&self, name: &str) -> Option<&Node> {
        if let Some(node) = self.hosts.iter()
                                .find(|&n| { n.name == name }) {
            return Some(node);
        } else if let Some(node) = self.switches.iter()
                                       .find(|&n| { n.name == name }) {
            return Some(node);
        }
        None
    }

    // Deploy the topology downto docker containers and docker networks.
    pub fn deploy(&self) -> Result<(), Box<dyn Error>> {
        for node in &self.hosts {
            DockerManager::create_node(&node)?;
        }
        for node in &self.switches {
            DockerManager::create_node(&node)?;
        }
        for link in &self.links {       // After all nodes created.
            DockerManager::create_link(&link)?;
        }
        Ok(())
    }

    // Stop and clean the running docker containers and networks.
    pub fn clean(&self) -> Result<(), Box<dyn Error>> {
        for link in &self.links {
            DockerManager::destroy_link(&link)?;
        }
        for node in &self.hosts {
            DockerManager::destroy_node(&node)?;
        }
        for node in &self.switches {
            DockerManager::destroy_node(&node)?;
        }
        Ok(())
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  Topo -")?;
        write!(f, "\n    Hosts:")?;
        for host in &self.hosts {
            write!(f, " {}", &host)?;
        }
        write!(f, "\n    Switches:")?;
        for switch in &self.switches {
            write!(f, " {}", &switch)?;
        }
        write!(f, "\n    Links:")?;
        for link in &self.links {
            write!(f, " {}", &link)?;
        }
        write!(f, "\n  In all, {} hosts, {} switches, {} links",
                  &self.hosts.len(), &self.switches.len(), &self.links.len())
    }
}


// Node (host / switch) in the network.
#[derive(Debug)]
pub struct Node {
    pub name: String,               // Name of the node in format: "h1".
}

impl Node {

    // Create a new node.
    pub fn new(name: &str) -> Node {
        Node { name: name.to_owned() }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


// Link that connects two nodes.
#[derive(Debug)]
pub struct Link {
    pub ends: (String, String),     // Name of two endpoints.
    pub name: String,               // Bridge name in format: "rusty-h1-s1".
}

impl Link {

    // Create a new link.
    pub fn new(ends: (&str, &str)) -> Link {
        Link {
            ends: (ends.0.to_owned(), ends.1.to_owned()),
            name: format!("{}-{}", ends.0, ends.1),
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}<->{})", self.ends.0, self.ends.1)
    }
}


// Config parsing error type.
#[derive(Debug, Clone)]
struct ConfigParsingError;

impl fmt::Display for ConfigParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something wrong when parsing the topology config file")
    }
}

impl Error for ConfigParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
