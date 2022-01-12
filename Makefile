build:
	docker-compose run build_node sh -c "cd distributed_nodes && cargo build --release"

clean:
	rm -rf distributed_nodes/target
	rm -rf distributed_nodes/Cargo.lock

all: clean build