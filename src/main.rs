use std::{fs, io};

fn methods() {
    mod some_module {
        pub struct MaxThree {
            counter: usize,
        }

        impl MaxThree {
            pub fn new() -> Self {
                Self { counter: 0 }
            }

            pub fn increase(&mut self) {
                if self.counter < 3 {
                    self.counter += 1;
                }
            }

            pub fn current(&self) -> usize {
                self.counter
            }
        }
    }

    println!("version 0 (raw)");
    let mut max_three = some_module::MaxThree::new();
    for n in 0..=4 {
        (&mut max_three).increase();
        println!("loop {}: counter={:?}", n, (&max_three).current());
    }

    println!("version 1");
    let mut max_three = some_module::MaxThree::new();
    for n in 0..=4 {
        max_three.increase();
        println!("loop {}: counter={:?}", n, max_three.current());
    }
}

fn range() {
    println!("version 0 (range)");
    let range = std::ops::RangeInclusive::new(1, 3);
    for n in range {
        println!("{}", n);
    }

    println!("version 1 (with ..= sugar)");
    for n in 1..=3 {
        println!("{}", n);
    }
}

fn arithmetic_shortcuts() {
    // version 0
    let mut foo = 1;
    foo = foo + 1;
    assert_eq!(foo, 2);

    // version 1
    let mut foo = 1;
    foo += 1;
    assert_eq!(foo, 2);
}

fn question_mark() -> io::Result<()> {
    // version 0
    let _ = match fs::read("/etc/os-release") {
        Ok(content) => content,
        Err(why) => return Err(why),
    };

    // version 1
    let _ = try!(fs::read("/etc/os-release"));

    // version 2
    let _ = fs::read("/etc/os-release")?;

    Ok(())
}

fn looping() {
    let three: &[i32] = &[1, 2, 3];

    println!("version 0");
    let mut index = 0;
    let count = three.len();
    loop {
        if count <= index {
            break;
        }
        println!("{}", three[index]);
        index += 1;
    }

    println!("version 1 (with 'while' keyword)");
    let mut index = 0;
    // len is a fundamental property of slice types...
    // 'three' is a slice (of i32)
    let count = three.len();
    while index < count {
        println!("{}", three[index]);
        index += 1;
    }

    println!("version 2 (with a trait)");
    let mut it = three.iter();
    while let Some(member) = it.next() {
        println!("{}", member);
    }

    println!("version 3 (same trait, but a functional approach)");
    three.into_iter().for_each(|member| println!("{}", member));

    println!("version 4 (with 'for' keyword, the sugar)");
    for member in three {
        println!("{}", member);
    }
}

fn run() -> io::Result<()> {
    range();
    arithmetic_shortcuts();
    question_mark()?;
    looping();
    methods();
    Ok(())
}

fn main() {
    if let Err(why) = run() {
        eprintln!("{}", why);
    }
}
