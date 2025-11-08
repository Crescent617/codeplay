/// This example demonstrates a simple lending iterator that yields lines from a string slice.
/// GAT: Generic Associated Types are used to define an associated type
trait LendingIterator {
    type Item<'a>
    where
        Self: 'a; // 可以将item的生命周期和迭代器本身绑定

    fn next(&'_ mut self) -> Option<Self::Item<'_>>; // 这个接口的定义能保证只有当item drop，才能advance (https://doc.rust-lang.org/nomicon/lifetime-mismatch.html)
}

struct Lines<'a> {
    data: &'a str,
    pos: usize,
}

impl<'a> LendingIterator for Lines<'a> {
    // 每个 item 都是一个切片，生命周期绑定在 self 的借用期
    type Item<'b> = &'b str where 'a: 'b;

    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        if self.pos >= self.data.len() {
            return None;
        }
        let rest = &self.data[self.pos..];
        let newline = rest.find('\n').unwrap_or(rest.len());
        let line = &rest[..newline];
        self.pos += newline + 1;
        Some(line)
    }
}


fn main() {
    let text = "Hello, world!\nThis is a test.\nLending iterators are cool!";
    let mut lines = Lines { data: text, pos: 0 };

    std::collections
    while let Some(line) = lines.next() {
        println!("{}", line);
    }
}
