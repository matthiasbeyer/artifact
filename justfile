# this is a comment

test: build
	cargo test --lib

filter PATTERN:
	cargo test --lib {{PATTERN}}

backtrace:
	RUST_BACKTRACE=1 cargo test --lib

build:
	cargo build

check:
	cargo check

version = `sed -En 's/version = "([^"]+)"/\1/p' Cargo.toml`

publish: clippy build
	git branch | grep '* master'
	git diff --no-ext-diff --quiet --exit-code
	cargo build --release --target=x86_64-unknown-linux-musl
	git add target/x86_64-unknown-linux-musl/release/just
	git commit -m "v{{version}} release"
	cargo publish
	git push origin master
	git tag -a "v{{version}}" -m "v{{version}}"
	git push origin --tags

clippy:
	rustup run nightly cargo clippy

install-clippy:
	rustup run nightly cargo install clippy

install-nightly:
	rustup install nightly

sloc:
	@cat src/*.rs | wc -l

# make a quine, compile it, and verify it
quine: create
	cc tmp/gen0.c -o tmp/gen0
	./tmp/gen0 > tmp/gen1.c
	cc tmp/gen1.c -o tmp/gen1
	./tmp/gen1 > tmp/gen2.c
	diff tmp/gen1.c tmp/gen2.c
	@echo 'It was a quine!'

quine-text = "int printf(const char*, ...); int main() { char *s = \"int printf(const char*, ...); int main() { char *s = %c%s%c; printf(s, 34, s, 34); return 0; }\"; printf(s, 34, s, 34); return 0; }"

# create our quine
create:
	mkdir -p tmp
	echo '{{quine-text}}' > tmp/gen0.c

# clean up
clean:
	rm -r tmp

# run all polyglot recipes
polyglot: python js perl sh ruby

python:
	#!/usr/bin/env python3
	print('Hello from python!')

js:
	#!/usr/bin/env node
	console.log('Greetings from JavaScript!')

perl:
	#!/usr/bin/env perl
	print "Larry Wall says Hi!\n";

sh:
	#!/usr/bin/env sh
	hello='Yo'
	echo "$hello from a shell script!"

ruby:
	#!/usr/bin/env ruby
	puts "Hello from ruby!"