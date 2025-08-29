fn median(list: &Vec<i32>) -> i32 {
    let mut list = list.clone();
    list.sort();
    return list[list.len() / 2];
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut counts = std::collections::HashMap::new();

    for n in list {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max = (0, 0);
    for (n, count) in counts {
        if count > max.1 {
            max.0 = n.clone();
            max.1 = count;
        }
    }
    return max.0;
}

fn main() {
    let mut l: Vec<i32> = vec![1, 45, 2, 435, 1, 325, 1, 23, 42, 325, 0, 7, 7, 50, 28];
    //                         0, 1, 1, 1, 2, 7, 7, 23, 28, 42, 45, 50, 325, 325, 435
    //                        mode^            median^

    println!("median: {}", median(&mut l));
    println!("mode: {}", mode(&l));

    dbg!(l);
}
