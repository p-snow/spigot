## -*- mode: makefile-gmake -*-

MAKEFLAGS += --no-print-directory

cargo:
	guix shell -m manifest.scm

build:
	guix shell -m manifest.scm -- cargo build

clean:
	guix shell -m manifest.scm -- cargo clean

run:
	guix shell -m manifest.scm -- cargo run

install:
	guix shell -m manifest.scm -- cargo install --path .

uninstall:
	guix shell -m manifest.scm -- cargo uninstall

test:
	guix shell -m manifest.scm -- cargo test
