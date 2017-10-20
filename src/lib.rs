extern crate laszip_sys;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

#[macro_use]
mod macros;

mod dll;
mod reader;
mod version;

pub use dll::Dll;
pub use reader::Reader;
pub use version::{Version, version};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// Wrapper around `std::ffi::NulError`.
        FfiNul(err: std::ffi::NulError) {
            from()
            cause(err)
            description(err.description())
            display("ffi nul error: {}", err)
        }
        /// Error from inside of the laszip library.
        Laszip(code: i32, message: String) {
            description("error from inside of the laszip library")
            display("laszip error {}: {}", code, message)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
