use std::fmt::Display;

fn main() {

    let longest = longest_with_an_announcement("hello", "world!", "最多字符串");

    println!("longest: {}", longest);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
