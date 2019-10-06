# RustyNet: Network Emulation over Cloud Docker Containers

Author: Guanzhou Hu @ MIT
Date: Oct 6, 2019


## Overview

**RustyNet** is a network emulator over Docker containers written in Rust. It is motivated by these two network emulators:

1. [Mininet](http://mininet.org/): the great SDN emulator, but only supports process-level isolation, and can not scale beyond a single host server.
2. [Yans](https://github.com/kennethjiang/YANS): a very basic prototype of doing network emulation over Docker containers in Python.

RustyNet's main goals lie in the following three aspects:

- Verify the feasibility of using the [*Rust*](https://www.rust-lang.org/) language to conduct network experiments.
- Verify the feasibility of doing network emulation over [*Docker containers*](https://www.docker.com/).
- (future) Scale such kind of emulation onto large-scale cloud containers orchestration platforms (e.g., [*Kubernetes*](https://kubernetes.io/)), and explore the advantages and drawbacks of doing it this way.

A demonstration of how RustyNet works:

GIF


## Installation

RustyNet requires the following prerequisites:

- [*Rust*](https://www.rust-lang.org/) toolchain - lastest stable version
- (optional in the future) *Vagrant* & *VirtualBox*
- (currently not needed) *Google Cloud CLI* & *Kubernetes* access

RustyNet only supports execution on Linux and OS X platforms.


## Usage

The following is a temporary usage guide based on local Vagrant environment. RustyNet's ultimate goal is to deploy such emulation onto cloud Docker containers in the future.

### Preparations

Under the project folder, do the following:

```Bash
# Bring the Vagrant VM up from the provided Vagrantfile
$ vagrant up
$ vagrant ssh

# Go into synced working directory.
$ cd /rustynet

# Compile in 'release' mode.
$ cargo build --release

# Build the Docker image for RustyNet nodes, from the provided Dockerfile.
$ cd docker-env
$ sudo docker build -t rustynet/node .
$ cd ..
```

### Run RustyNet

Initial topology is generated from a YAML config file `<topo-name>.yml`. Put it under `topolib/` folder. (Currently this prototype always used the `minimal.yml` example topology.)

Run RustyNet **in root user privilege**:
```Bash
$ sudo ./target/release/rustynet
```


## TODO List

- [x] Prototyping on local Docker containers
- [ ] Tweak resource & bandwidth limits in Docker
- [ ] Publish the Docker image online & doing pull instead of real-time build
- [ ] Verify the effect of using host bridges as links
- [ ] Better way of emulating "routers" using containers
- [ ] More example topologies
- [ ] Scale it up onto cloud platforms
