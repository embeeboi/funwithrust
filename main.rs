
use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::BTreeSet;


fn main()
{
    let rust_boi = true;
    if rust_boi == true
    {
        println!("Rust is pretty cool");
    }

    let mut cars = HashMap::new();
    cars.insert("BMW", 1);
    cars.insert("Audi", 2);
    cars.insert("Ferrari", 3);
    

    let mut linkstuff = LinkedList::new();
    linkstuff.push_front(3);

    let mut btreeboi = BTreeSet::new();
    btreeboi.insert(43);
}