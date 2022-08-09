#![no_std]
#![no_main]

//extern crate panic_halt;

#[no_mangle]
pub unsafe extern "C" fn rust_function() -> u8
    {
    0
    //println!("Hello World! from Rust");
    }

#[no_mangle]
pub unsafe extern "C" fn rust_giveback_u8(a: u8) -> u8
    {
    a
    }

#[no_mangle]
pub unsafe extern "C" fn rust_pfunction() -> u8
    {
    0
    //println!("new lib")
    }    

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! 
    {
        loop {}
    }
