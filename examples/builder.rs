use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
pub struct Drink {
    #[builder(setter(into))]
    name: String,
    #[builder(default = "456")]
    sugar_level: i32,
    #[builder(default = "123")]
    price: u8,
    #[builder(default = "vec![]", setter(each(name = "unknown", into)))]
    ingredients: Vec<String>,
    #[builder(setter(into, strip_option), default)]
    from: Option<String>,

    #[builder(setter(custom))]
    produce_date: i64,
    #[builder(setter(custom))]
    produce_date1: DateTime<Utc>,
}

impl Drink {
    pub fn build() -> DrinkBuilder {
        DrinkBuilder::default()
    }
}

impl DrinkBuilder {
    fn produce_date(&mut self, value: i64) {
        self.produce_date = Some(value);
    }

    fn produce_date1(&mut self, value: &str) -> anyhow::Result<()> {
        let p_date = chrono::DateTime::parse_from_rfc3339(value)?.to_utc();
        self.produce_date1 = Some(p_date);
        anyhow::Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut builder = Drink::build();
    let builder = builder
        .name("jasmine coffee")
        .sugar_level(1)
        .price(8)
        .from("UloveMe")
        .ingredients(vec!["coffee".to_string()]);

    builder.produce_date(20240519);
    builder.produce_date1("2024-05-19T15:45:45Z")?;

    let drink = builder.build()?;

    // Drink { name: "jasmine coffee", sugar_level: 1, price: 8, ingredients: ["coffee"], from: Some("UloveMe"), produce_date: 20240519, produce_date1: 2024-05-19T15:45:45Z }
    println!("{:?}", drink);
    anyhow::Ok(())
}
