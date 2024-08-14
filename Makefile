all: build

build:
	cargo build --release
install: build
	sudo cp target/release/crust /usr/bin
uninstall:
	sudo rm -f usr/bin/crust
clean:
	cargo clean
.PHONY: all build install uninstall clean
