use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    age: u8,
    skills: Vec<String>,
    #[serde(rename = "myState")]
    status: MyState,
    #[serde(serialize_with = "b64_encode")]
    sec_code: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(rename_all = "snake_case")]
enum MyState {
    Learn(String),
    WorkOn(Vec<String>),
}

fn main() -> anyhow::Result<()> {
    let state = MyState::WorkOn(vec!["Learning Rust".to_string(), "Write blogs".to_string()]);

    let user = Person {
        name: "alex".to_string(),
        age: 28,
        skills: vec!["programming".to_string()],
        status: state,
        sec_code: vec![1, 2, 3, 6, 5],
    };

    // {"name":"alex","age":28,"skills":["programming"],"myState":{"workOn":["Learning Rust","Write blogs"]},"secCode":"AQIDBgU"}
    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    anyhow::Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}
