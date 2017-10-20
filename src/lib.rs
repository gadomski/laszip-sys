extern crate laszip_sys;
#[macro_use]
extern crate quick_error;

#[macro_use]
mod macros;

mod version;

pub use version::{Version, version};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// Error from inside of the laszip library.
        Laszip(code: i32, message: String) {
            description("error from inside of the laszip library")
            display("laszip error {}: {}", code, message)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
