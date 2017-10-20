macro_rules! laszip_try_without_pointer {
    ($expr:expr) => {{
        use ::Error;
        let result = $expr;
        if result > 0 {
            return Err(Error::Laszip(result, "no error message".to_string()));
        }
    }}
}
