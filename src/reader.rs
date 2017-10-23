use {Dll, Result};
use laszip_sys;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

/// Reads las data.
#[derive(Debug)]
pub struct Reader {
    is_compressed: bool,
    pointer: *mut c_void,
    _dll: Dll,
}

impl Reader {
    /// Opens a reader for a provided path.
    ///
    /// # Examples
    ///
    /// ```
    /// use laszip::Reader;
    /// let reader = Reader::from_path("data/autzen.laz").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader> {
        use std::ffi::CString;

        let path = CString::new(path.as_ref().to_string_lossy().as_ref())?;
        let mut is_compressed = 0;
        let mut pointer = ptr::null_mut();
        let dll = Dll::load()?;
        unsafe {
            laszip_try_without_pointer!(laszip_sys::laszip_create(&mut pointer));
            laszip_try!(
                laszip_sys::laszip_open_reader(pointer, path.as_ptr(), &mut is_compressed),
                pointer
            );
        }
        Ok(Reader {
            is_compressed: is_compressed != 0,
            pointer: pointer,
            _dll: dll,
        })
    }

    /// Returns true if this reader's data is compressed.
    ///
    /// Laszip readers can be used to read uncompressed laz data.
    ///
    /// # Examples
    ///
    /// ```
    /// use laszip::Reader;
    /// let reader = Reader::from_path("data/autzen.laz").unwrap();
    /// assert!(reader.is_compressed());
    /// let reader = Reader::from_path("data/autzen.las").unwrap();
    /// assert!(!reader.is_compressed());
    /// ```
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

    #[test]
    fn is_compressed() {
        let reader = Reader::from_path("data/autzen.laz").unwrap();
        assert!(reader.is_compressed());
        let reader = Reader::from_path("data/autzen.las").unwrap();
        assert!(!reader.is_compressed());
    }
}
