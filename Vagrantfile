# Deployment environment for RustyNet.


# Provision script.
$setup = <<-SCRIPT
    cd $HOME
    sudo apt-get update
    sudo apt-get upgrade
    sudo apt-get autoremove
    sudo apt-get install -y \
        build-essential \
        apt-transport-https \
        ca-certificates \
        curl \
        software-properties-common \
        git \
        bridge-utils \
        python3 \
        python3-dev \
        python3-pip \
        vim \
        libssl-dev

    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
    sudo add-apt-repository \
        "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
        $(lsb_release -cs) \
        stable"
    sudo apt-get update
    sudo apt-get -y install docker-ce docker-ce-cli containerd.io

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
SCRIPT


# Vagrant configuration.
Vagrant.configure("2") do |config|

    # Distro: Ubuntu 16.04.
    config.vm.box = "bento/ubuntu-18.04"

    # Connection with external env.
    config.vm.synced_folder ".", "/rustynet"
    config.vm.network :forwarded_port, guest:8000, host:8000    # Left blank.

    # Virtual box configurations.
    config.vm.provider "virtualbox" do |vb|
        vb.gui = false
        vb.cpus = 4
        vb.memory = 8192
        vb.customize [
            "modifyvm", :id, "--uartmode1", "disconnected"  # Disable logs.
        ]
    end

    # Vagrant up provision script.
    config.vm.provision "shell", privileged: false, inline: $setup
end
