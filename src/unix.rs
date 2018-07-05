use libc;

use std::ptr;
use std::mem;
use std::slice;

fn getpwuid(buffer: &mut [i8;512]) -> Result<libc::passwd, String> {
    let mut pwent: libc::passwd = unsafe { mem::zeroed() };
    let mut pwent_p = ptr::null_mut();

    // The char pointers referenced by the struct passwd fields will point into the buffer,
    // which will have the strings copied into it
    let retcode;
    unsafe {
        retcode = libc::getpwuid_r(libc::geteuid(), &mut pwent, &mut buffer[0], buffer.len(), &mut pwent_p);
    }

    // Return value of 0 is normally success, but if it hasn't set the pointer value then it
    // couldn't find the user in the /etc/passwd database
    if retcode == 0 && pwent_p.is_null() {
        return Err("User not found in passwd db".to_string())
    }
    if retcode != 0 {
        return Err("Other error".to_string())
    }

    Ok(pwent)
}

pub fn get_username() -> String {
    let mut buffer = [0i8;512];
    let pwent = getpwuid(&mut buffer).unwrap();

    let uname = pwent.pw_name as *mut u8;
    let namestring;

    unsafe {
        let slice = slice::from_raw_parts(uname, libc::strlen(pwent.pw_name));
        namestring = String::from_utf8_lossy(slice).to_string();
    }

    namestring
}
