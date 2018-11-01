fn move_example() {
    let s1 = String::from("hello"); // instantiate variable
    let _s2 = s1;                   // value moves into _s2

    println!("{}, world!", s1);     // s1 is invalid because value moved
}

fn clone_example() {
    let s1 = String::from("hello"); // instantiate variable
    let _s2 = s1.clone();           // clone s1 into _s2

    println!("{} {}, world!", s1, _s2); // both are valid
}

fn main() {
    clone_example();

    let mut s1 = String::from("hello");
    // {
        let s2 = &s1;
        println!("{}", s2);
    // }

    s1 = s1.replace("h", "y");

    println!("{}", s1);
}
