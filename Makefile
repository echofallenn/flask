.PHONY: all build release clean fmt check install

all: build

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean
	rm -rf target/cxx

fmt:
	cargo fmt

check:
	cargo clippy -- -D warnings

install: release
	@echo "Installing flask to /usr/local/bin..."
	install -m 755 target/release/flask /usr/local/bin/flask
	@echo "Done. Run 'flask --help' to get started."

uninstall:
	rm -f /usr/local/bin/flask
