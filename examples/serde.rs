use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    skills: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum MyState {
    Init(String),
    Running(Vec<String>),
    Done(u32),
}

fn main() -> anyhow::Result<()> {
    let user = Person {
        name: "alex".to_string(),
        age: 28,
        skills: vec!["programming".to_string()],
    };

    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let d_user: Person = serde_json::from_str(&json)?;
    println!("{:?}", d_user);
    assert_eq!(user, d_user);

    let state = MyState::Running(vec![
        "Todo".to_string(),
        "PENDING".to_string(),
        "WIP".to_string(),
    ]);
    let state_json = serde_json::to_string(&state)?;
    println!("state_json: {}", state_json);

    anyhow::Ok(())
}
