pub struct Foo {
    x: u64,
}

impl Foo {
    /// Any type that borrows an instance of Foo can
    /// call this method, as it only requires a reference to Foo.
    pub fn total(&self) -> u64 {
        self.x
    }
    /// Only exclusive references to instances of Foo
    /// can call this method, as it requires Foo to be mutable.
    pub fn increase(&mut self) {
        self.x += 1;
    }
}

let foo = Foo { x: 10 };
println!("{}", foo.total()) // WORKS.
foo.increase() // ERROR: Foo not declared as mut