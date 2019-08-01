fn main() {
    let test = measure_persistence(13);
    println!("{}", test);
    let test2 = measure_persistence(1234);
    println!("{}", test2);
    let test3 = measure_persistence(9876);
    println!("{}", test3);
    let test4 = measure_persistence(199);
    println!("{}", test4);
}

fn reduce(number: isize) -> isize {
    let mut vec: Vec<isize> = vec![];
    let mut new_number = number;
    while new_number > 0 {
        if new_number >= 10 {
            let digit = number % 10;
            new_number /= 10;
            vec.push(digit);
        } else {
            vec.push(new_number);
            new_number = 0;
        }
    }

    return vec.iter().sum();
}

fn measure_persistence(number: isize) -> isize {
    let mut persistence = 0;
    let mut cur_num = number;
    while cur_num >= 10 {
        cur_num = reduce(cur_num);
        persistence += 1;
    }
    return persistence;
}
