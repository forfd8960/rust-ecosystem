use serde::Serialize;
use serde_json;
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator, VariantNames};

#[derive(Debug, EnumCount, EnumIter, VariantNames)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Display, PartialEq, Serialize)]
enum Color {
    #[strum(serialize = "red_color", to_string = "#FF2400")]
    Red,

    // Notice that we can disable certain variants from being found
    #[strum(disabled)]
    Yellow,
}

fn main() -> anyhow::Result<()> {
    let sunday = Week::Sunday;
    println!("{:?}", sunday);
    println!("Week count: {:?}", Week::COUNT);

    println!("{:?}", Week::VARIANTS);
    Week::iter().for_each(|x| println!("{:?}", x));

    let color_red = Color::Red;
    println!("{:?}", color_red);

    let red_v = serde_json::to_string(&color_red)?;
    println!("color_red: {:?} to string: {}", color_red, red_v);

    let color_y = Color::Yellow;
    println!("{:?}", color_y);

    let v = serde_json::to_string(&color_y)?;
    println!("color_y: {:?} to string: {}", color_y, v);
    anyhow::Ok(())
}
