VERSION:= v0.1.0

all: clean sign trust

trust:
	if [[ ! -f generator-$(VERSION) ]]; then \
		make -B sign || exit 1; \
	fi
	ln -sf generator-$(VERSION) generator

generator:
	cd generator-native && cargo build --release
	ln -sf generator-native/target/release/generator-native generator
	cp -f `readlink generator` ./generator-$(VERSION)

sign: generator
	sha256sum ./generator-$(VERSION) | cut -d\  -f1 > generator-$(VERSION).sha256
	gpg --sign ./generator-$(VERSION)

clean:
	rm -f generator-v*

sfexec-output:
	g++ -Iinclude/ -w --std=c++17 main.cpp sha256_literal/sha256.cpp -o sfexec
