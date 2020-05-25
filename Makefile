
generator:
	cd generator-native && cargo build --release
	ln -sf generator-native/target/release/generator-native generator

sfexec-output:
	g++ -Iinclude/ -w --std=c++17 main.cpp sha256_literal/sha256.cpp -o sfexec
