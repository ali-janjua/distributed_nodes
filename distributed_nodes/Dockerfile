FROM rust:1.57
COPY target/release/distributed_nodes /bin/distributed_nodes
RUN apt-get update
RUN apt-get install --assume-yes net-tools
RUN apt-get install --assume-yes telnet
RUN apt-get install --assume-yes iputils-ping
CMD ["/bin/distributed_nodes"]
