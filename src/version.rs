use {Dll, Result};
use laszip_sys;

/// Returns the dll version for laszip.
///
/// # Examples
///
/// ```
/// let version = laszip::version().unwrap();
/// ```
pub fn version() -> Result<Version> {
    let mut major = 0;
    let mut minor = 0;
    let mut revision = 0;
    let mut build = 0;
    unsafe {
        let _dll = Dll::load()?;
        laszip_try_without_pointer!(laszip_sys::laszip_get_version(
            &mut major,
            &mut minor,
            &mut revision,
            &mut build,
        ));
    }
    Ok(Version {
        major: major,
        minor: minor,
        revision: revision,
        build: build,
    })
}

/// A structure to hold the laszip version.
#[derive(Clone, Copy, Debug)]
pub struct Version {
    /// The dll major version.
    pub major: u8,
    /// The dll minor version.
    pub minor: u8,
    /// The dll revision.
    pub revision: u16,
    /// The dll build number.
    pub build: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        super::version().unwrap();
    }

    #[test]
    fn multithreaded_version() {
        use std::thread;
        for _ in 0..10 {
            thread::spawn(|| super::version().unwrap());
        }
    }
}
