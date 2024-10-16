use iterators::print_vec;

fn main() {

    let v1: Vec<i32> = vec![1, 2, 3];
    print_vec(&v1);
    let v2: Vec<_> = v1.iter().map(|x| x + 3).collect();
    print_vec(&v2);

}

