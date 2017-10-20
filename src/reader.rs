use {Dll, Result};
use laszip_sys;
use std::path::Path;
use std::ptr;

pub struct Reader {
    _dll: Dll,
    is_compressed: bool,
}

impl Reader {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader> {
        use std::ffi::CString;

        let path = CString::new(path.as_ref().to_string_lossy().as_ref())?;
        let mut is_compressed = 0;
        let dll = Dll::load()?;
        unsafe {
            let mut pointer = ptr::null_mut();
            laszip_try_without_pointer!(laszip_sys::laszip_create(&mut pointer));
            laszip_try!(
                laszip_sys::laszip_open_reader(pointer, path.as_ptr(), &mut is_compressed),
                pointer
            );
        }
        Ok(Reader {
            _dll: dll,
            is_compressed: if is_compressed == 0 { false } else { true },
        })
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path() {
        let reader = Reader::from_path("data/autzen.laz").unwrap();
        assert!(reader.is_compressed());
    }
}
