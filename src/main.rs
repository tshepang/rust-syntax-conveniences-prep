use std::{fs, io};

fn type_elision() {
    let value: Vec<u8> = (1_u8..=3).collect();
    println!("explicit: {:?}", value);
    let value: Vec<_> = (1..=3).collect();
    println!("elided: {:?}", value);
}

fn type_elision_shorthand() {
    struct Number(i32);
    impl Number {
        fn increase(self: &mut Self) {
            self.0 += 1;
        }
        fn increase_elided(&mut self) {
            self.0 += 1;
        }
    }
    let mut number = Number(0);
    number.increase();
    println!("explicit increment: {}", number.0);
    number.increase_elided();
    println!("elided increment: {}", number.0);
    println!("'&mut self' ~-> 'self: &mut Self'");
    println!("'&self' ~-> 'self: &Self'");
    println!("'self' ~-> 'self: Self'");
}

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
    for n in std::ops::RangeInclusive::new(1, 3) {
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
    let _ = fs::read("/etc/os-release")?;

    Ok(())
}

fn looping() {
    let three: &[i32] = &[1, 2, 3];

    println!("version 0 (looping");
    let mut index = 0;
    let count = three.len();
    // len is a fundamental property of slice types,
    // 'three' is a slice (of array of i32)
    loop {
        if index == count {
            break;
        }
        println!("{}", three[index]);
        index += 1;
    }

    println!("version 1 (with 'while' keyword)");
    let mut index = 0;
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

    println!("version 3 (with 'for' keyword, the sugar)");
    for member in three {
        println!("{}", member);
    }

    println!("version 4 (addendum: a functional approach)");
    three.iter().for_each(|member| println!("{}", member));
}

fn no_derive() {
    println!("version 0 (manual)");
    struct Point {
        x: isize,
        y: isize,
    }
    use std::fmt;
    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
    let point = Point { x: 1, y: 2 };
    println!("{:#?}", point);
}

fn derive() {
    println!("version 1 (with 'derive' annotation)");
    #[derive(Debug)]
    struct Point {
        x: isize,
        y: isize,
    }
    let point = Point { x: 1, y: 2 };
    println!("{:#?}", point);
}

fn lifetime_elision() {
    fn not_elided<'a>(value: &'a str) {
        println!("{}", value);
    }
    fn elided<'a>(value: &str) {
        println!("{}", value);
    }
    println!("version 0 (not elided)");
    not_elided("some value");
    println!("version 1 (elided)");
    elided("some value");
    const FOO: &'static str = r#"const "&'static" explicit"#;
    const BAR: &str = r#"const "&'static" elided"#;
    println!("{} | {}", FOO, BAR);
    static BAZ: &'static str = r#"static "&'static" explicit"#;
    static QUX: &str = r#"static "&'static" elided"#;
    println!("{} | {}", BAZ, QUX);
}

fn type_inference_and_coercion() {
    let mut explicit: Vec<f32> = Vec::new();
    explicit.push(0.1_f32);
    println!("explicit: {:?}", explicit);
    let mut inferred = Vec::new();
    inferred.push(0.1_f32);
    println!("inferred: {:?}", inferred);

    let foo: i64 = 1_64;
    println!("veri explicit... no inference or coercion: {:#X}", -foo);
    let foo: i8 = 1;
    println!("coerced to i8: {:#X}", -foo);
    let foo = 1;
    assert_eq!(foo, 1_i16);
    println!("inferred as i16: {:#X}", -foo);
    let foo = 1;
    assert_eq!(foo, 1); // no coercion... default is i32
    println!("default (fallback) signed type is i32: {:#X}", -foo);
    let foo: i64 = 1_i64;
    println!("no inference or coercion: {:#X}", -foo);
}

fn deref_coercion() {
    let vec = vec![1, 2, 3];
    let arr = [1, 2, 3];
    let equal = vec == arr;
    assert!(equal);
}

fn hidden_code() {
    println!("
automatically inserted:
 #[macro_use]
 extern crate std;
 use std::prelude::v1::*;
    ");
    println!("\
Also done for us automatically:
- eh_personality: runs destructors in case of panic (stack unwinding)
- panic_handler: a function to be called in case of panic
- start: setup function called before main()
    ");
}

fn if_let() {
    let maybe = (1..3).next();
    println!("version 0");
    match maybe {
        Some(value) => println!("{:?}", value),
        None => (),
    };
    println!("version 0 (if let)");
    if let Some(value) = maybe {
        println!("{:?}", value);
    };
}

fn pretty_imports() {
    println!("
to avoid repetition, one can do this:
 use std;:path::{{self, Path, PathBuf}};
compare that to this:
 use std::path;
 use std::path::Path;
 use std::path::PathBuf;
    ");
}

fn implicit_return() {
    fn explicit_return() -> i32 {
        if true {
            return 0;
        } else {
            return 1;
        }
    }
    fn implicit_return() -> i32 {
        if true {
            0
        } else {
            1
        }
    }
    fn _implicit_return_compact() -> i32 {
        if true { 0 } else { 1 }
    }
    fn _explicit_return_compact() -> i32 {
        if true { return 0; } else { return 1; }
    }
    assert_eq!(explicit_return(), implicit_return());
}

fn run() -> io::Result<()> {
    range();
    arithmetic_shortcuts();
    question_mark()?;
    looping();
    methods();
    no_derive();
    derive();
    lifetime_elision();
    type_inference_and_coercion();
    deref_coercion();
    hidden_code();
    if_let();
    type_elision();
    type_elision_shorthand();
    pretty_imports();
    implicit_return();
    deref_coercion();
    Ok(())
}

fn main() {
    if let Err(why) = run() {
        eprintln!("{}", why);
    }
}
