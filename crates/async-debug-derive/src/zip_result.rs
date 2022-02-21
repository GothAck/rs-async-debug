pub trait ZipResult<T, E> {
    fn zip_result<U>(self, second: Result<U, E>) -> Result<(T, U), E>;
}

impl<T, E> ZipResult<T, E> for Result<T, E> {
    fn zip_result<U>(self, second: Result<U, E>) -> Result<(T, U), E> {
        match self {
            Err(e) => Err(e),
            Ok(first) => match second {
                Err(e) => Err(e),
                Ok(second) => Ok((first, second)),
            },
        }
    }
}
