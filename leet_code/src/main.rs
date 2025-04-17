mod one;
mod seventeensixtyeight;

fn main() {
    // let sum: Vec<i32> = one::two_sum([1, 2, 3, 4, 5].to_vec(), 9);

    let sum: String = seventeensixtyeight::merge_alternately("abc".to_string(), "pqr".to_string()); 

    println!("{:?}", sum);
}


