use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    skills: Vec<String>,
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
    anyhow::Ok(())
}
