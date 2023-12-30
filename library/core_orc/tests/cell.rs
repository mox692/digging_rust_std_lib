use core_orc::cell::*;

#[test]
fn cell_into_inner() {
    let cell = Cell::new(42);
    assert_eq!(cell.into_inner(), 42);
}

#[test]
fn cell_set() {
    let cell = Cell::new(42);
    cell.set(45);
    assert_eq!(cell.into_inner(), 45);
}

#[derive(Debug, PartialEq)]
struct Foo {
    pub value: i32,
}
struct Bar {
    pub foo: RefCell<Foo>,
}
#[test]
fn get_immutable_reference_from_inner_value() {
    let ref_cell = RefCell::new(Foo { value: 32 });
    let s1 = ref_cell.borrow();
    let s2 = ref_cell.borrow();
    assert_eq!(s1.value, 32);
    assert_eq!(s1.value, s2.value);
}

#[test]
fn get_mutable_reference_from_inner_value() {
    let bar = Bar {
        foo: RefCell::new(Foo { value: 32 }),
    };

    let foo2 = Foo { value: 1 };
    *bar.foo.borrow_mut() = foo2;

    assert!(bar.foo == RefCell::new(Foo { value: 1 }));
}

#[test]
fn should_panic_when_both_mutable_and_immutable_reference_exist_at_the_same_time() {}
