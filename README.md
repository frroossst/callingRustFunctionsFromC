## Introduction

I wanted to do some funky type conversions, for a piece of hardware that needed to be robust and not crash,
well, C is kinda not really memory safe, what is? Rust! But running Rust on embedded hardware is finnicky and
libraries and community support is hard to come by (although this has improved massively since I started this project), and interfaces with the hardware sensors are a nightmare to say the least.  

**This is meant to be more of a template rather than a tutorial**

## Roadmap

1. ~Dynamically link a Rust lib and C code on x86~

2. ~Statically link a Rust lib and C code on x86~

3. Cross compile a simple led C code for arm target

4. Cross compile a simple led Rust code for arm target

5. ~Cross compile and statically link a compiled Rust lib with C code~

6. ~Replicate on other machines to confirm procedures~

7. Return an array to a C function

## Simplest and the quickest way to go about it 

1. You can use platformIO with VSCode which is much easier to do when using things like `Serial.print()` or `Delay()` for LED blinks etc.  

2. Put the compiled Rust library in lib subfolder in the project folder  

3. In the platform.ini add build flags  
    `build_flags = =l<libname> -Llib/`    

## The harder way about 

Here are the steps I took to compile a rust library for the teensy 4.1 and statically linked it to a main.c file

1. Install rust and other tools (rustup, cargo etc.)

2. Add the compilation target (thumbv7em-none-eabi)  
    `rustup target add thumbv7em-none-eabi`   
    `rustup target add thumbv7em-none-eabi --toolchain stable`  

3. Change up your Rust boilerplate code to the following  
    `#![no_std]` this makes it so that the stdlib is not included in the compiled library  
    `#![no_mangle]` this will not make changes to the functon name so that we can call Rust functions from C/C++  

    We build using `cargo build --target thumbv7em-none-eabi`  

4. Create a .cargo/config file and add the following  
    ```[build]
    target = "thumbv7em-none-eabi"
    ```

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
        > NOTE: linked libraries for embedded devices without an operating system will need the static flag  
        
        `gcc -o output main.c -l<libname> -L<libpath . for curr> --static`  
        `clang-10 -o output main.c -l<libname> -L<libpath . for curr> --static`

2. Statically link using arm gcc
    `arm-linux-gnueabi-gcc-9 -static main.c -o static.out -labi -L.`

3. Create a hex file using arm gcc  
    `arm-linux-gnueabi-objcopy -O ihex <source file> <output file>`


## Other

Resources  
https://www.youtube.com/watch?v=kgW56enMVek
https://www.youtube.com/watch?v=rOMl9FHgZ_0

Additional Read  
https://stackoverflow.com/questions/37912290/arm-linux-executable-mysteriously-runs-on-x86-64

Credits  

