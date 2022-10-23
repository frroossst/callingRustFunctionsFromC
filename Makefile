build:
	cargo build --release
	gcc main.c -o output -labi -L./target/release/ -lm --static
	./output
