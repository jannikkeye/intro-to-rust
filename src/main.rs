use std::fmt;
use std::time::Duration;
use std::thread::sleep;
mod example_mod_file;
mod example_mod_folder;

mod example_basic {
    use super::*;
    #[derive(Debug)]
    pub struct Struct {
        string: &'static str,
        ch: char,
    }

    impl Struct {
        pub fn new() -> Self {
            Struct {
                string: "'ello!",
                ch: 'H',
            }
        }
    }

    impl fmt::Display for Struct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.string.replace("'", &self.ch.to_string()))
        }
    }
}

fn main() {
    let example_basic = example_basic::Struct::new();
    let example_mod_file = example_mod_file::Struct::new();
    let example_mod_folder = example_mod_folder::Struct::new();

    loop {
        println!("{} from example_basic\n", example_basic);
        println!("{} from example_mod_file\n", example_mod_file);
        println!("{} from example_mod_folder\n", example_mod_folder);
        sleep(Duration::from_millis(1000));
    }
}
