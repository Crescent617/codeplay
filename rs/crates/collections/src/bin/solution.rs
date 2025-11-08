#![allow(unused_macros)]

macro_rules! input {
    () => {{
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim().to_owned()
    }};
}

macro_rules! timeit {
    ($block: tt) => {{
        use std::time;
        let dur = time::Duration::from_secs(1);
        let t = time::Instant::now();
        let mut loop_num = 0;
        loop {
            loop_num += 1;
            $block;
            if t.elapsed() > dur {
                break;
            }
        }
        let dt = t.elapsed();
        println!(
            "run {} loops, mean time usage: {:?}",
            loop_num,
            dt / loop_num
        );
    }};

    ($name: expr, $block: tt) => {{
        println!("Test Name: {}", $name);
        timeit!($block);
    }};
}

macro_rules! input_num {
    () => {{
        input!().parse().unwrap()
    }};
}

macro_rules! input_nums {
    () => {{
        input!()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }};
}

macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut dict = std::collections::HashMap::new();
        $( dict.insert($key, $val); )*
        dict
    }};
}

fn main() {}

struct Solution;
