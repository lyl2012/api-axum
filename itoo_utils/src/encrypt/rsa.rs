use std::str::from_utf8;

use rsa::{pkcs8::FromPrivateKey, Hash, PaddingScheme, RsaPrivateKey};

use sha2::{Digest, Sha256};

use super::super::Error;

//签名 支持的hash方式
pub enum HashType {
    // Sha1,
    Sha256,
}

pub struct Rsa {
    //私钥
    priv_key: RsaPrivateKey,
    //公钥
    //pub_key: RsaPublicKey,
    //签名的hash方式
    hash_type: HashType,
}

impl Rsa {
    //创建实例
    pub fn new(priv_key: &str, hash_type: HashType) -> Result<Self, Error> {
        //私钥
        let p_key = format_pri_key(&priv_key)?;
        //私钥对象
        let priv_key = RsaPrivateKey::from_pkcs8_pem(&p_key)?;

        let s = Self {
            priv_key,
            //pub_key,
            hash_type,
        };
        Ok(s)
    }

    //对数据进行签名
    pub fn sign<'a>(&self, data: &'a str) -> Result<String, Error> {
        let hash_type = match &self.hash_type {
            // HashType::Sha1 => Hash::SHA1,
            HashType::Sha256 => Hash::SHA2_256,
        };

        //要签名的数据，需要先进行摘要计算.
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());

        let r = hasher.finalize().as_slice().to_vec();
        let sign = self.priv_key.sign(
            PaddingScheme::PKCS1v15Sign {
                hash: Some(hash_type),
            },
            &r,
        )?;
        let s = base64::encode(&sign);
        Ok(s.clone())
    }
}

fn chunks_list(key: &str) -> Result<String, Error> {
    let subs_list = key
        .as_bytes()
        .chunks(64)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()?;

    Ok(subs_list.join("\n"))
}
// 格式化私钥
fn format_pri_key(priv_key: &str) -> Result<String, Error> {
    if priv_key.starts_with("-") {
        return Ok(priv_key.to_string());
    }
    let list = chunks_list(priv_key)?;

    let mut data = String::default();
    let prefix = "-----BEGIN PRIVATE KEY-----\n";
    let subfix = "\n-----END PRIVATE KEY-----";
    data += prefix;
    data += &list.as_str();
    data += subfix;

    Ok(data)
}
