use std::result::Result as StdResult;
use std::error::Error as ErrorTrait;

#[warn(clippy::all)]

pub type BError = Box(Error);

pub type Result<T, E = BError> = StdResult(T, E);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorType {

    // connection errors
    ConnectionTimeout,
    ConnectionRefused,
    ConnectNoRoute,
    TLSHandshakeFailure,
    TLSHandshakeTimeout,
    InvalidCert,

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
