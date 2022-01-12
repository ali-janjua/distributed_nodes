FROM rust:1.57
COPY distributed_nodes/target/release/distributed_nodes /bin/distributed_nodes

CMD ["/bin/distributed_nodes"]
