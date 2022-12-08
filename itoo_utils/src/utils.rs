use crate::Error;
use serde::Serialize;
use serde_json::Value;

//生成待签名字符串
pub fn generate_sign_str<T>(data: Option<T>) -> Result<String, Error>
where
    T: Serialize + Send,
{
    if let Some(body) = data {
        //报文转换为json结构，进行签名前处理
        let json_value = serde_json::to_value(body)?;
        let pre = String::default();
        let assemble_data: Vec<String> = assemble(pre, json_value);
        let sign = assemble_data.join("&");
        Ok(sign)
    } else {
        Ok(String::default())
    }
}

//格式化待签名的报文
fn assemble(pre: String, param: Value) -> Vec<String> {
    match param {
        Value::Bool(v) => vec![v.to_string()],
        Value::String(v) => {
            let ss = format!("{}={}", pre.clone(), v);
            vec![ss]
        }
        Value::Null => vec![],
        Value::Number(v) => {
            let ss = format!("{}={}", pre.clone(), v.to_string());
            vec![ss]
        }
        Value::Array(v) => {
            let mut sign: Vec<String> = Vec::new();
            //数组 只取最后一个参与签名
            let last = v.last().unwrap();
            let mut ss = assemble(pre.clone(), last.clone());
            sign.append(&mut ss);
            // for vv in v {
            //     let mut ss = get_sign_str(vv, pre.clone());
            //     sign.append(&mut ss)
            // }
            sign
        }
        Value::Object(v) => {
            let mut sign: Vec<String> = Vec::new();

            for (i, pv) in v {
                let p = format!("{}{}", &pre, &i);
                let mut ss = assemble(p, pv);
                sign.append(&mut ss);
            }
            sign
        }
    }
}
