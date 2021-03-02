use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum ResultData<T> {
    Ok(HackOk<T>),
    Err(RequestError),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct HackOk<T> {
    #[serde(flatten)]
    inner: T,
    result: HackResultSucess,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RequestError {
    result: HackResultError,
    msg: String,
    code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum HackResultSucess {
    Success,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum HackResultError {
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn fail1() {
        let json = r#"{
            "msg": "Invalid API key",
            "result": "error"
        }
        "#;
        let x: ResultData<Value> = serde_json::from_str(json).unwrap();
        assert_eq!(
            x,
            ResultData::Err(RequestError {
                code: None,
                msg: "Invalid API key".into(),
                result: HackResultError::Error
            })
        )
    }

    #[test]
    fn fail2() {
        let json = r#"{
            "msg": "Invalid URL",
            "result": "error",
            "code": "418: ENOURL"
        }
        "#;
        let x: ResultData<Value> = serde_json::from_str(json).unwrap();
        assert_eq!(
            x,
            ResultData::Err(RequestError {
                code: Some("418: ENOURL".into()),
                msg: "Invalid URL".into(),
                result: HackResultError::Error
            })
        )
    }

    #[test]
    fn fail3() {
        let json = r#"{"result":"error","msg":"Invalid email 'user6x@zulipdev.com'","code":"BAD_REQUEST"}"#;
        let x: ResultData<Value> = serde_json::from_str(json).unwrap();
        assert_eq!(
            x,
            ResultData::Err(RequestError {
                code: Some("BAD_REQUEST".into()),
                msg: "Invalid email 'user6x@zulipdev.com'".into(),
                result: HackResultError::Error
            })
        )
    }

    #[test]
    fn sucess1() {
        let json = r#"{"result":"success","msg":"","id":104}"#;

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Data {
            id: i32,
        }

        let x: ResultData<Data> = serde_json::from_str(json).unwrap();
        assert_eq!(
            x,
            ResultData::Ok(HackOk {
                inner: Data { id: 104 },
                result: HackResultSucess::Success
            })
        );
    }
}
