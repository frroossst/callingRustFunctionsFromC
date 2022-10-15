#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn get_fram_writeable_temp(temp :i8) -> u8
    {
    // checking bounds : -100C to 100C
    // int u8 0 reserved as Error code
    let upper_limit :i8 = 100;
    let lower_limit :i8 = -100;

    if (temp > upper_limit) & (temp < lower_limit)
        {
        0
        }
    else
        {
        (temp + 101).try_into().unwrap()
        }
    }


