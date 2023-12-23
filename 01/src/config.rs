use std::collections::HashMap;

pub fn init() -> HashMap<&'static str, u32> {
    let mut needles: HashMap<&str, u32> = HashMap::new();
    needles.insert("one", 1);
    needles.insert("two", 2);
    needles.insert("three", 3);
    needles.insert("four", 4);
    needles.insert("five", 5);
    needles.insert("six", 6);
    needles.insert("seven", 7);
    needles.insert("eight", 8);
    needles.insert("nine", 9);
    needles.insert("1", 1);
    needles.insert("2", 2);
    needles.insert("3", 3);
    needles.insert("4", 4);
    needles.insert("5", 5);
    needles.insert("6", 6);
    needles.insert("7", 7);
    needles.insert("8", 8);
    needles.insert("9", 9);

    return needles;
}
