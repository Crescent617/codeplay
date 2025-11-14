struct MyBox<T> {
    value: *const T,
    _marker: std::marker::PhantomData<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox {
            value: Box::into_raw(Box::new(value)),
            _marker: std::marker::PhantomData,
        }
    }
}

fn main() {
    let x = String::from("Hello, world!");
    let my_box = MyBox::new(x);

    println!("{}", &x);
    println!("{}", unsafe { &*my_box.value });
}
