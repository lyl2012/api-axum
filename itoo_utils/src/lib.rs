mod encrypt;
mod error;
mod utils;

pub use error::Error;
pub use utils::generate_sign_str;

mod demo {
    use crate::encrypt::{self, rsa::Rsa};

    //RSA demo
    fn demo_rsa() -> Result<String, crate::Error> {
        //私钥
        let priv_key = "this is a test private key";
        let rsa = Rsa::new(priv_key, encrypt::rsa::HashType::Sha256)?;
        rsa.sign("encrypt data")
    }
}
