NAME=rustymon
IMAGE=gcr.io/gibbsdevops/$(NAME)

export RUST_LOG=rustymon=debug

# include Makefile.local

run:
	cargo run

build:
	cargo build --release

docker-build:
	docker build . -t $(IMAGE)
	docker run --rm $(IMAGE) ls -alh /usr/local/bin/rustymon

docker-run:
	docker run --rm -it --name $(NAME) $(IMAGE)

docker-push:
	docker push $(IMAGE)
