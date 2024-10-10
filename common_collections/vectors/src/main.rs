fn main() {
    let mut v: Vec<i32> = Vec::new();

    // or

    //let mut vec2 = vec![1, 2, 3];

    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![100, 32, 57];

    for i in 0..4{
        let element: Option<&i32> = v.get(i);
        match element {
            Some(element) => println!("vec[{i}] = {element}"),
            None => println!("There is no {} elements",i+1),
        };
    }
    // v.get(i) returns v[i] if v[i] is defined and None if v[i] is not defined

    for i in &mut v2 {
        *i /= 2;        // modify the value of v[0], v[1], ... by dereferencing
    }

    for i in &v2 {
        println!("{i}");
    }

}
