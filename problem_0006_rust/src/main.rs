



fn main() {

    let n: i64 = 100;
    let sum_of_n_squares: i64 = (1..(n + 1)).map(|x| i64::pow(x, 2)).sum();
    let square_of_n_sum = i64::pow((1..(n + 1)).sum(), 2);
    let difference_n = square_of_n_sum - sum_of_n_squares;


    println!(
        "The difference is {} - {} = {}",
            square_of_n_sum, 
            sum_of_n_squares,
            difference_n
    );
}
