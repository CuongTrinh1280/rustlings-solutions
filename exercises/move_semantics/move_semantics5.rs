// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    print_type_of(&*y);
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
