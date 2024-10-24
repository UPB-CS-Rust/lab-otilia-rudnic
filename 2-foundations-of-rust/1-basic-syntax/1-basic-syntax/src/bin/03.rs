fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let n = input.len() - 1;
    let mut smallest = input[0];
    let mut largest = input[n];
    for i in 0..n {
        if input[i] < smallest {
            smallest = input[i];
        }
        if input[i] > largest {
            largest = input [i];
        }
    }
    println!("{} is largest and {} is smallest", largest, smallest);
}
