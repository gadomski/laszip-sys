use {Dll, Result};
use laszip_sys;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

pub struct Reader {
    is_compressed: bool,
    pointer: *mut c_void,
    _dll: Dll,
}

impl Reader {
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

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn point_count(&self) -> Result<i64> {
        let mut count = 0;
        unsafe {
            laszip_try!(
                laszip_sys::laszip_get_point_count(self.pointer, &mut count),
                self.pointer
            );
        }
        Ok(count)
    }

    pub fn is_indexed(&self) -> Result<bool> {
        self.has_spatial_index().map(|(is_indexed, _)| is_indexed)
    }

    fn has_spatial_index(&self) -> Result<(bool, bool)> {
        let mut is_indexed = 0;
        let mut is_appended = 0;
        unsafe {
            laszip_try!(
                laszip_sys::laszip_has_spatial_index(self.pointer, &mut is_indexed, &mut is_appended),
                self.pointer
            );
        }
        Ok((is_indexed != 0, is_appended != 0))
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
    fn point_count() {
        let reader = Reader::from_path("data/autzen.laz").unwrap();
        assert_eq!(0, reader.point_count().unwrap());
    }

    #[test]
    fn is_indexed() {
        let reader = Reader::from_path("data/autzen.laz").unwrap();
        assert!(!reader.is_indexed().unwrap());
    }
}
