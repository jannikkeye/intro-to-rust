use std::fmt;

fn main() {
    // TYPES
    println!("Hello, world!\n");

    println!("Check these types:");
    
    println!("u8, u16, u32, u64, usize:     {}", 123);
    println!("i8, i16, i32, i64, isize:     {}", -123);
    println!("f32, f64:                     {}", 123.25);
    println!("string slice:                 {}", "It's me, a string!");
    println!("array:                        {:?}", [0, 1, 2]);
    println!("boolean:                      {}", true);
    println!("char:                         {}", 'a');
    println!("tuple:                        {:?}", ("a", "b"));
    println!("slice                         {:?} written as {1}", &[0, 1, 3, 4, 5, 6][2..5], "&[0, 1, 3, 4, 5, 6][2..5]");

    // STRUCTS

    println!("\nCheck this struct:\n");

    #[derive(Debug)]
    struct Struct {
        u_8: u8,
        i_8: i8,
        f_32: f32,
        string: &'static str,
        array: [u8; 3],
        boolean: bool,
        ch: char,
        tuple: (u8, char),
        slice: &'static [u8]
    }

    let example = Struct {
        u_8: 8,
        i_8: -8,
        f_32: 8.0,
        string: "'ello!",
        array: [0, 1, 2],
        boolean: true,
        ch: 'H',
        tuple: (8, 'h'),
        slice: &[0, 1, 2][1..2]
    };

    println!("{:#?}", example);

    impl Struct {
        pub fn new(array: [u8; 3]) -> Self {
            Struct {
                u_8: 8,
                i_8: -8,
                f_32: 8.0,
                string: "'ello!",
                array,
                boolean: true,
                ch: 'H',
                tuple: (8, 'h'),
                slice: &[0, 1, 2][1..2]
            }
        }
    }

    let example_2 = Struct::new([3, 2, 4]);

    println!("{:#?}\n", example_2);

    // TRAITS

    impl fmt::Display for Struct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.string.replace("'", &self.ch.to_string()))
        }
    }

    println!("Normal debug log: {:?}\n", example);
    println!("Trait in action: {}\n", example);
}
