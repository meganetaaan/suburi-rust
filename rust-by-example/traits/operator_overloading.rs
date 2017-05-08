use std::ops;

struct Foo{foo: i32}
struct Bar{bar: i32}

#[derive(Debug)]
struct FooBar{foo: i32, bar: i32}

#[derive(Debug)]
struct BarFoo{foo: i32, bar: i32}

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar{foo: self.foo, bar: _rhs.bar}
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo{bar: self.bar, foo: _rhs.foo}
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo{foo: 1} + Bar{bar: 2});
    println!("Bar + Foo = {:?}", Bar{bar: 3} + Foo{foo: 4});
}
