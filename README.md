I wanted to do some funky type conversions, for a piece of hardware that needed to be robust and not crash,
well, C is kinda not really memory safe, what is? Rust! But running Rust on embedded hardware is finnicky and
libraries and community support is hard to come by, and interfaces with the hardware sensors are a nightmare to
say the least.

Here are the steps I took to compile a rust library for the teensy 4.1 and statically linked it to a main.c file

1. Install rust and other tools (rustup, cargo etc.)

2. Add the compilation target (thumbv7em-none-eabi)
    `rustup target add thumbv7em-none-eabi`
    `rustup target add thumbv7em-none-eabi --toolchain stable`
    `rustup install stable`
    `rustup default stable`
    `rustup update stable`
    `rustup toolchain install nightly`
    `rustup override set beta`
    `rustup target add thumbv7em-none-eabi`


3. Change up your Rust boilerplate code to the following
    #![no_Std]
    #![no_mangle]
    Add panic-halt to Cargo.toml
    extern crate panic_halt;

We build using `carog build --target thumbv7em-none-eabi`

4. Create a .cargo/config file and add the following
    [build]
    target = "thumbv7em-none-eabi"

5. Compile C code to arm
    `sudo apt-get install gcc-arm-linux-gnueabi`
    `sudo apt install gcc-arm-none-eabi`


## How to statically link Rust lib with C code on x86

1. Create a Rust lib
    * Rename main.rs to lib.rs
    * Add function with the following syntax
        ```
        #[no_mangle]
        pub extern "C" fn rust_test() -> u8
            {
            0
            }
        ```
    * Change Cargo.toml to include
        ```
        [lib]
        crate-type = ["staticlib"]

        [dependencies]
        panic-halt = "0.2.0"
        ```
    * Compile and link
        `gcc -o output main.c -l<libname> -L<libpath . for curr>`



Resources


