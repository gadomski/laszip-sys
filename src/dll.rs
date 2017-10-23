use Result;
use laszip_sys;
use std::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
}

#[derive(Debug)]
pub struct Dll {}

impl Dll {
    pub fn load() -> Result<Dll> {
        let mut counter = COUNTER.lock().unwrap();
        if *counter == 0 {
            unsafe {
                laszip_try_without_pointer!(laszip_sys::laszip_load_dll());
            }
        }
        *counter += 1;
        Ok(Dll {})
    }
}

impl Drop for Dll {
    fn drop(&mut self) {
        let mut counter = COUNTER.lock().unwrap();
        *counter -= 1;
        if *counter == 0 {
            unsafe {
                let result = laszip_sys::laszip_unload_dll();
                if result != 0 {
                    panic!("error when unloading laszip dll: {}", result);
                }
            }
        }
    }
}
