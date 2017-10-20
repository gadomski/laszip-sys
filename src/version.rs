use Result;
use laszip_sys;

pub fn version() -> Result<Version> {
    let mut major = 0;
    let mut minor = 0;
    let mut revision = 0;
    let mut build = 0;
    unsafe {
        laszip_try_without_pointer!(laszip_sys::laszip_load_dll());
        laszip_try_without_pointer!(laszip_sys::laszip_get_version(
            &mut major,
            &mut minor,
            &mut revision,
            &mut build,
        ));
        laszip_try_without_pointer!(laszip_sys::laszip_unload_dll());
    }
    Ok(Version {
        major: major,
        minor: minor,
        revision: revision,
        build: build,
    })
}

#[derive(Clone, Copy, Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub revision: u16,
    pub build: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        super::version().unwrap();
    }
}
