struct Test;

impl Test {
    fn borrow(&self) {}
    fn borrow_mut(&mut self) {}
    fn consume(self) {}
}

fn is_fn<F: Fn()>(_f: F) {}
fn is_fn_mut<F: FnMut()>(_f: F) {}
fn is_fn_once<F: FnOnce()>(_f: F) {}

fn main() {
    let mut test = Test;

    // let _fn = || test.borrow();
    // let mut _fn_mut = || test.borrow_mut();
    let _fn_consume = || test.consume(); // FnMut

    is_fn(_fn_consume);
}
