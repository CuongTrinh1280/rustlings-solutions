// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(value) => value,
        }
    }
}

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let word = optional_word {
        println!("The word is: {}", word.unwrap());
    } else {
        println!("The optional word doesn't contain anything");
    }

    let optional_integers_vec: &mut Vec<Option<i8>> = &mut Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    println!("{:#?}", optional_integers_vec.into_iter()
                                    .filter_map(|v| v.as_ref())
                                    .collect::<Vec<_>>()
            );

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {:?}", integer);
    }

    let nested_optional = optional_integers_vec.pop();
    let flattening = nested_optional.flatten();
    println!("{:?}", flattening);
}
