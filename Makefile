COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
BUILDER = ewbankkit/rust-amazonlinux:1.41.1-2018.03.0.20191219.0

.PHONY: all amazonlinux build check clean doc release test update

all: build

amazonlinux:
	@docker run --volume $(PWD):/volume --rm --tty $(BUILDER) cargo build --release

build:
	@$(CARGO) build

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

release:
	@$(CARGO) build --release

test: build
	@$(CARGO) test

update:
	@$(CARGO) update
