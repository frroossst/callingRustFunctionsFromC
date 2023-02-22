.PHONY: *

compile_target=thumbv7em-none-eabihf
# if the library is called libabi.a then the libname is abi
libname=abi

help:
	@echo "callingRustFunctionsFromC"
	@echo "uncomment the toolchain specific commands in the Makefile to compile for non x86_64 targets\n"
	@echo "make build"
	@echo "    build the Rust library"
	@echo "make link"
	@echo "    link the Rust library with the C code"
	@echo "make run"
	@echo "    run the C code"


run:
	./output

clean:
	rm output
	rm -r target/

build:
	cargo build --release --verbose
	# cargo build --release --target=$(compile_target) --verbose

link:
	cargo build --release
	# cargo build --release --target=$(compile_target)
	gcc main.c -o output -labi -L./target/release --static
	# arm-none-eabi-gcc main.c -o output -l$(libname) -L./target/$(compile_target)/release --static

hex:
	arm-none-eabi-objcopy -O ihex output output.hex
