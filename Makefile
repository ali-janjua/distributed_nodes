build-code:
	docker-compose run build_node sh -c "cd distributed_nodes && cargo build --release"

build-container:
	cd distributed_nodes && docker build -t distributed_node .

build: build-code build-container
clean:
	rm -rf distributed_nodes/target
	rm -rf distributed_nodes/Cargo.lock

all: build