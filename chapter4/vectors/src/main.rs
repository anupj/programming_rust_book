fn main() {
    let v = vec!["liberte".to_string(),
                 "egalite".to_string(),
                 "fraternite".to_string()];


    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    // --- Rc --- //
    use std::rc::Rc;

    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavour", u);

    // A value owned by an Rc pointer is immutable
    // so you can't do the following
    // s.push_str(" noodles "); <- this will error out
}
