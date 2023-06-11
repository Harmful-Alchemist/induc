#![feature(generic_const_exprs)]

fn main() {
    let zero = No::new();
    zero.equal(No::new());

    // zero.equal(No::new().increment()); // compile error

    zero.increment().equal(No::new().increment());

    gt(No::new().increment(), No::new());
    // gt(No::new(),No::new().increment()); //compile error

    gt(
        No::new().add(No::new().increment().increment()),
        No::new().increment(),
    );
}

struct No<const N: usize> {}

impl No<0> {
    pub fn new() -> No<0> {
        No {}
    }
}

impl<const N: usize> No<N> {
    pub fn equal(&self, _: No<N>) {}

    pub fn increment(&self) -> No<{ N + 1 }> {
        No {}
    }

    pub fn add<const M: usize>(&self, _: No<M>) -> No<{ N + M }> {
        No {}
    }
}

fn gt<const N: usize, const M: usize>(_: No<N>, _: No<M>) -> No<{ N - M - 1 }> {
    No {}
}
