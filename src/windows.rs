pub fn get_username() -> String {
    extern "system" {
        // From Winbase.h:
        // BOOL GetUserNameW(LPWSTR lpBuffer, LPDWORD pcbBuffer);
        fn GetUserNameW(a: *mut u16, b: *mut usize) -> i32;
    }

    let mut name = [0u16;256];
    let mut size = 256;

    // size will hold the size of the buffer including the terminating null
    unsafe { GetUserNameW(&mut name[0], &mut size); }

    String::from_utf16_lossy(&name[..size-1])
}
