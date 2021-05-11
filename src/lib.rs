#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(test)]
extern crate tempfile;

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindings::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use tempfile::NamedTempFile;

    #[test]
    fn example_one() {
        unsafe {
            assert_eq!(0, laszip_load_dll());

            let mut version_major = 0;
            let mut version_minor = 0;
            let mut version_revision = 0;
            let mut version_build = 0;
            assert_eq!(
                0,
                laszip_get_version(
                    &mut version_major,
                    &mut version_minor,
                    &mut version_revision,
                    &mut version_build,
                )
            );

            let mut reader = ptr::null_mut();
            assert_eq!(0, laszip_create(&mut reader));

            let mut is_compressed = 0;
            let path = CString::new("data/autzen.laz").unwrap();
            assert_eq!(
                0,
                laszip_open_reader(reader, path.as_ptr(), &mut is_compressed,)
            );
            assert_eq!(1, is_compressed);

            let mut header = ptr::null_mut();
            assert_eq!(0, laszip_get_header_pointer(reader, &mut header));

            let npoints = if (*header).number_of_point_records == 0 {
                (*header).extended_number_of_point_records
            } else {
                (*header).number_of_point_records as u64
            };
            assert_eq!(110000, npoints);

            let mut point = ptr::null_mut();
            assert_eq!(0, laszip_get_point_pointer(reader, &mut point));

            let mut writer = ptr::null_mut();
            assert_eq!(0, laszip_create(&mut writer));

            assert_eq!(0, laszip_set_header(writer, header));

            let outfile = NamedTempFile::new().unwrap();
            let path = CString::new(outfile.path().to_string_lossy().as_ref()).unwrap();
            assert_eq!(0, laszip_open_writer(writer, path.as_ptr(), 1));

            for _ in 0..npoints {
                assert_eq!(0, laszip_read_point(reader));
                assert_eq!(0, laszip_set_point(writer, point));
                assert_eq!(0, laszip_write_point(writer));
            }

            assert_eq!(0, laszip_close_writer(writer));
            assert_eq!(0, laszip_destroy(writer));
            assert_eq!(0, laszip_close_reader(reader));
            assert_eq!(0, laszip_destroy(reader));
        }
    }
}
