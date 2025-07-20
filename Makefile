build:
	cargo build --release
install: build
	sudo mv ./target/release/phresh /usr/bin/phresh
	
