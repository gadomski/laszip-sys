macro_rules! laszip_try_without_pointer {
    ($expr:expr) => {{
        use ::Error;
        let result = $expr;
        if result != 0 {
            return Err(Error::Laszip(result, "no error message".to_string()));
        }
    }}
}

macro_rules! laszip_try {
    ($expr:expr, $pointer:expr) => {{
        use ::Error;
        use ::laszip_sys;
        use std::ffi::CStr;

        let result = $expr;
        if result != 0 {
            let mut error = ptr::null_mut();
            let result2 = laszip_sys::laszip_get_error($pointer, &mut error);
            if result2 != 0 {
                return Err(Error::Laszip(result, "while retriving error message".to_string()));
            } else {
                return Err(Error::Laszip(result, CStr::from_ptr(error).to_string_lossy().into_owned()));
            }
        }
    }}
}
