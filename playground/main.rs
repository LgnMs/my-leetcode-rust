fn main() {
    let a = ["1", "2", "lol"];

    let mut iter = a.iter().filter_map(|s| s.parse().ok()).collect::<Vec<u32>>();
    print!("{:?}", iter);
}
