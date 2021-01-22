use std::{collections::HashMap, iter};

use typed_builder::TypedBuilder;

macro_rules! extend {
    [$init:expr; $($expr:expr),*$(,)?] => {{
        let mut e = $init;
        $(e.extend(iter::once($expr));)*
        e
    }};
}

#[derive(PartialEq, TypedBuilder)]
struct Foo {
    // Mandatory Field:
    x: i32,

    // #[builder(default)] without parameter - use the type's default
    // #[builder(setter(strip_option))] - wrap the setter argument with `Some(...)`
    #[builder(default, setter(strip_option))]
    y: Option<i32>,

    // Or you can set the default
    #[builder(default = 20)]
    z: i32,

    // #[builder(default)] without parameter - don't require this field
    // #[builder(setter(strip_collection))] without parameter - start with the default and extend from there
    #[builder(default, setter(strip_collection))]
    v0: Vec<i32>,

    // No `default`: This field must be set at least once.
    // You can explicitly create the collection from the first item (but this is not required even without `default`).
    #[builder(setter(strip_collection(from_first = |first| vec![first])))]
    v1: Vec<i32>,

    // Other `Extend` types are also supported.
    #[builder(default, setter(strip_collection))]
    h: HashMap<i32, i32>,
}

fn main() {
    assert!(
        Foo::builder().x(1).y(2).z(3).v0(4).v1(5).h((6, 7)).build()
            == Foo {
                x: 1,
                y: Some(2),
                z: 3,
                v0: vec![4],
                v1: vec![5],
                h: extend![HashMap::new(); (6, 7)],
            }
    );

    // Change the order of construction:
    assert!(
        Foo::builder().z(1).x(2).h((3, 4)).v1(5).v0(6).y(7).build()
            == Foo {
                x: 2,
                y: Some(7),
                z: 1,
                v0: vec![6],
                v1: vec![5],
                h: extend![HashMap::new(); (3, 4)],
            }
    );

    // Optional fields are optional:
    assert!(
        Foo::builder().x(1).v1(2).build()
            == Foo {
                x: 1,
                y: None,
                z: 20,
                v0: vec![],
                v1: vec![2],
                h: HashMap::new(),
            }
    );

    // Extend fields can be set multiple times:
    assert!(
        Foo::builder().x(1).v0(2).v0(3).v0(4).v1(5).v1(6).h((7, 8)).h((9, 10)).build()
            == Foo {
                x: 1,
                y: None,
                z: 20,
                v0: vec![3, 4],
                v1: vec![5, 6],
                h: extend![HashMap::new(); (7, 8), (9, 10)],
            }
    );

    // This will not compile - because we did not set x:
    // Foo::builder().build();

    // This will not compile - because we set y twice:
    // Foo::builder().x(1).y(2).y(3);
}
