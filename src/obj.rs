use crate::common::{Vec2, Vec3};

pub fn tokenize(text: &str) -> Vec<Vec<&str>> {
    let a = Vec2::new(2.0, 2.0);
    text.split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}
