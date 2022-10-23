#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn my_panic(_info: &PanicInfo) -> !
    {
    loop {}
    }

#[no_mangle]
pub extern "C" fn give_an_u8() -> u8
    {
    10
    }

#[no_mangle]
pub extern "C" fn pass_and_get_an_u8(x: u8) -> u8
    {
    x + 1
    }

#[no_mangle]
pub extern "C" fn get_an_array_pointer(a: u8, b: u8) -> *mut u8
     {
     let mut arr: [u8; 2] = [a, b];
     let ptr = arr.as_mut_ptr();
     ptr
     }
