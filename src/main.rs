/// # Sandbox
/// This is the start of the insane sandbox test playground for rust where
/// I attempt to learn the very cool funky low level language.
fn main() {
    println!("Hello, world!");
    let names = vec!["Bob", "Frank", "Ferris"];
    let mut names2 = vec![String::from("Bob"), String::from("Frank"), String::from("Ferris")];
    println!("This is iter");

    // You get immutable references to the contents of the names vec.
    for (index, name) in names.iter().enumerate() {
        println!("{}", index);
        println!("{}", name);
    }
    println!("{}", names.len());

    // This destroys the names variable because into_iter() consumes the object and the
    // ownership is moved to the iterator.
    println!("This is into_iter");
    for name in names.into_iter() {
        println!("{}", name);
    }

    // Unlike into_iter, it doesn't consume a variable but if the contents within the vec
    // are mutable, they can be altered. For strings, you should note that above we have another
    // variable named "names2". This variable
    println!("This is mut_iter");
    for name in names2.iter_mut() {
        name.push_str(" stuff");
        println!("{}", name);
    }
}