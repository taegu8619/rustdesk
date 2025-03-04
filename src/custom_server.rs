use hbb_common::{
    bail,
    base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _},
    sodiumoxide::crypto::sign,
    ResultType,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct CustomServer {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub api: String,
    #[serde(default)]
    pub relay: String,
}

// ✅ 기본적으로 사용자 지정 서버를 반환하는 함수 추가
pub fn get_custom_server() -> CustomServer {
    CustomServer {
        host: "211.35.78.106".to_string(),
        key: "D3Q423eGber4QQ1kAGYLqaoV23SgbotzRzkCZYGfryGJv264zfxkDe8NKX4ilWovtVlmUdxnu1B2VKAylKqdkQ==".to_string(),
        api: "".to_string(),
        relay: "".to_string(),
    }
}

fn get_custom_server_from_config_string(s: &str) -> ResultType<CustomServer> {
    let tmp: String = s.chars().rev().collect();
    const PK: &[u8; 32] = &[88, 168, 68, 104, 60, 5, 163, 198, 165, 38, 12, 85, 114, 203, 96, 163, 70, 48, 0, 131, 57,
        12, 46, 129, 83, 17, 84, 193, 119, 197, 130, 103];
    let pk = sign::PublicKey(*PK);
    let data = URL_SAFE_NO_PAD.decode(tmp)?;
    
    if let Ok(lic) = serde_json::from_slice::<CustomServer>(&data) {
        return Ok(lic);
    }
    
    if let Ok(data) = sign::verify(&data, &pk) {
        Ok(serde_json::from_slice::<CustomServer>(&data)?)
    } else {
        bail!("sign:verify failed");
    }
}

pub fn get_custom_server_from_string(s: &str) -> ResultType<CustomServer> {
    // ✅ 기본적으로 사용자 지정 서버 반환 (파일명 분석 없이)
    return Ok(get_custom_server());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_custom_server() {
        let server = get_custom_server();
        assert_eq!(server.host, "211.35.78.106");
        assert_eq!(server.key, "D3Q423eGber4QQ1kAGYLqaoV23SgbotzRzkCZYGfryGJv264zfxkDe8NKX4ilWovtVlmUdxnu1B2VKAylKqdkQ==");
        assert_eq!(server.api, "");
        assert_eq!(server.relay, "");
    }
}
