use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    println!("the length is {}", table.len());
    for(_artist, works) in table {
        works.sort();
    }
}

fn add_one(num: &mut i32) {
    *num += 1;
}        

fn main() {
    let mut table = Table::new();
    // Create entries for table
    table.insert("Gesualdo".to_string(),
                 vec!["Many Madrigals".to_string(), 
                         "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(), 
                         "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(), 
                         "A Salt Cellar".to_string()]);

    show(&table);

    let x = 10;
    let r = &x;       // &x is a shared reference to x
    assert!(*r == 10);      // explicitly dereference r
    let mut y = 32;
    let m = &mut y;         // mutable reference to y
    *m += 32;    // explicitly dereference m to set y's value
    assert!(*m == 64 );

    let mut num = 64;
    add_one(&mut num);
    assert_eq!(num, 65);
    sort_works(&mut table);

    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.y, 729);

}
