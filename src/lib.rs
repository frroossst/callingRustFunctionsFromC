#![no_std]

#[no_mangle]
pub unsafe extern "C" fn rust_function()
    {
    println!("Hello World! from Rust");
    }

#[no_mangle]
pub unsafe extern "C" fn rust_giveback_u8(a: u8) -> u8
    {
    a
    }

#[no_mangle]
pub unsafe extern "C" fn rust_pfunction()
    {
    println!("new lib")
    }    
