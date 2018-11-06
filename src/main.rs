fn clone_example() {
    let s1 = String::from("hello"); // instantiate variable
    let s2 = s1.clone();           // clone s1 into s2

    println!("{} {}, world!", s1, s2); // both are valid
}

fn scope_example() {
    let mut s1 = String::from("hello");
    {
        let s2 = &s1;
        println!("{}", s2);
    } // s2 goes out of scope and memory gets cleared

    // println!("{}", s2);
    s1 = s1.replace("h", "y");

    println!("{}", s1);
}

// fn move_example() {
//     let s1 = String::from("hello"); // instantiate variable
//     let _s2 = s1;                   // value moves into _s2

//     println!("{}, world!", s1);     // s1 is invalid because value moved
// }


// There can always only be one mutable reference
// fn mutable_reference() {
//     let mut s1 = String::from("hello");
//     let s2 = &mut s1;
//     let s3 = &mut s1;   

//     println!("{} {} {}", &s1, s2, s3);
// }

fn main() {
    clone_example();
    scope_example();
}
