trait Doubler {
    fn double(&self, data: u32) -> u32;
}

struct First; // size 0

#[derive(Default)]
struct Second {
    data: [u8; 30],
}

struct Third;

impl Doubler for First {
    fn double(&self, data: u32) -> u32 {
        data * 2
    }
}

impl Doubler for Second {
    fn double(&self, data: u32) -> u32 {
        data * 2
    }
}

fn main() {
    // direct usage
    let x = First;
    let y = Second::default();

    // foo::<First>
    let a = x.double(5);
    assert_eq!(10, a);

    // foo::<Second>
    let b = y.double(10);
    assert_eq!(20, b);

    foo(&x);
    foo(&y);

    foo_dyn(&x);
    foo_dyn(&y);


    let j = First;
    let k = First;

    foo_arr(&[j, k]);

    let jt = Third;
    let kt = Third;

    let j2 = First;
    let k2 = First;

    let i: [&dyn Doubler; 2] = [&j2, &k2];

    // dyn Doubler is unsized!

    // foo_arr(&[jt, kt]);
    foo_dyn_arr(&i);

    let j3 = First;
    let k3 = Second::default();

    let j: [&dyn Doubler; 2] = [&j3, &k3];

    box_foo_dyn(Box::new(First));


    // T: Doubler => T is sized, Doubler is a Trait, T is the type
    // dyn Doubler => `dyn Doubler` is NOT sized, Doubler is a trait, `dyn Doubler` is a type
    // &dyn Doubler =>
    //   * `dyn Doubler` is NOT sized, but &dyn Doubler IS!
    //   * Doubler is still the trait
    //   * `dyn Doubler` is still a type, we just have a reference to it (like &T).
}

// As a "capability constraint" for generic functions
fn foo<T: Doubler>(doubler: &T) {
    let j = 5;
    let k = doubler.double(j);
    assert_eq!(k, 10);
}

fn foo_arr<T: Doubler>(doubler: &[T]) {
    let j = 5;
    let k = doubler[0].double(j);
    assert_eq!(k, 10);
}

// As a "capability constraint" for generic functions
fn foo_dyn(doubler: &dyn Doubler) {
    let j = 5;

    // RIGHT HERE
    let k = doubler.double(j);
    assert_eq!(k, 10);
}

fn box_foo_dyn(doubler: Box<dyn Doubler>) {
    let j = 5;

    // RIGHT HERE
    let k = doubler.double(j);
    assert_eq!(k, 10);
}



fn foo_dyn_arr(doubler: &[&dyn Doubler]) {
    let j = 5;

    // RIGHT HERE
    let k = doubler[0].double(j);
    assert_eq!(k, 10);
}
