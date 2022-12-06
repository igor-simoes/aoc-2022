use std::fs::read_to_string;

fn max_calories() {
    let data = read_to_string("../data/day1.txt").expect("file not found");
    let result: u32 = data
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|item| !item.is_empty())
                .map(|y| y.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("{:?}", result);
    assert_eq!(result, 71124)
}

fn sum_top3() {
    let data = read_to_string("../data/day1.txt").expect("file not found");
    let mut result: Vec<u32> = data
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|item| !item.is_empty())
                .map(|y| y.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    result.sort_by(|a, b| b.cmp(a)); // inverse sort
    result.truncate(3);  // get only the first three elements

    let top_three_sum = result.iter().sum::<u32>();

    println!("{:?}", top_three_sum);
    assert_eq!(top_three_sum, 204639)
}