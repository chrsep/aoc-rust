mod test;

fn main() {
    println!("Hello, world!");
}

fn day_one(input: String) {
    let values = input.split("\n");
    let numbers  = values.map(|value| {
        return value.parse::<i32>().unwrap();
    }).collect::<Vec<i32>>();
    println!("{:?}", numbers)
}
