

extern crate ndarray;

use ndarray::prelude::*;

fn two_references<T: Copy>(a: &mut T, b: &mut T) {
    /* Rust requires that a and b will never point to the same value:
     * mutable access is unique access */
    println!("In two references with reference {:p} and {:p}", a, b);
    *a = *b;
}

fn get_some_element<'a, T>(view: &mut ArrayViewMut2<'a, T>) -> &'a mut T {
    view.elem_ref_mut([1, 1])
}

#[test]
fn test() {
    let mut data = [1, 2, 3, 4];
    let mut view = ArrayViewMut::from(&mut data).into_shape((2, 2)).unwrap();

    let element_1 = get_some_element(&mut view);
    let element_2 = get_some_element(&mut view);

    two_references(element_1, element_2);
}
