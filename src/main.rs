mod test;

fn main() {
    println!("Hello, world!");
}

fn day_one(input: String) -> i32 {
    let values = input.split("\n");
    let mut numbers = values
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    numbers.sort();
    println!("{:?}", numbers);

    let result = search(numbers, 1721);
    println!("{:?}", result);
    return 2;
}

fn search(input: Vec<i32>, value: i32) -> i32 {
    let length = input.len();
    return binary_search(input, value, length / 2);
}

fn binary_search(input: Vec<i32>, value: i32, pivot: usize) -> i32 {
    let mut target = pivot;
    if pivot > input.len() - 1 {
        target = input.len() - 1;
    };

    if input[target] == value {
        return target as i32;
    }

    if target == 0 || target == input.len() - 1 {
        return -1;
    }

    return if input[target] < value {
        binary_search(input, value, target + (target / 2))
    } else {
        binary_search(input, value, target / 2)
    };
}