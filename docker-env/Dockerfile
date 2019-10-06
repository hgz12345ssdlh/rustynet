FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y iproute2 \
                       tcpdump \
                       vim \
                       radvd \
                       iputils-ping \
                       kmod \
                       net-tools \
                       mtr-tiny \
                       traceroute \
                       netcat \
                       dnsutils \
                       curl

CMD ["bash"]