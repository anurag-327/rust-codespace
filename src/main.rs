use std::collections::HashMap;
use std::collections::HashSet;
#[allow(unused_variables)]
#[allow(unused_imports)]
use std::io;
fn data_types() {
    println!("Data types in rust!");
    let x: i8 = -2;
    println!("x ={}", x);
    let x: u8 = 2;
    println!("x ={}", x);
    let x: f32 = 3.21;
    println!("x ={}", x);
    let mut x: String = String::new();
    x.push_str("Hello");
    x.push_str(" World!");
    println!("x ={}", x);
    let decimal: f64 = 329.32;
    let y: u16 = decimal as u16;
    println!("y ={}", y);
    let arr: [i16; 5] = [1, 2, 3, 4, 5];
    println!("array ={:?}", arr);
    let arr = [2, 3, 4, 5, 6];
    println!("array ={:?}", arr);
    let mut arr: [i32; 5] = [3; 5];
    println!("array ={:?}", arr);
    arr[1] = 2;
    arr[3] = 4;
    println!("array ={:?}", arr);
    let slice = &arr[1..3];
    println!("Sliced Array ={:?}", slice);
    let slice = &arr[..3];
    println!("Sliced Array ={:?}", slice);
    let slice = &arr[1..];
    println!("Sliced Array ={:?}", slice);
    let slice = &mut arr[1..3];
    println!("Sliced Array ={:?}", slice);
    slice[0] = 8;
    println!("Mutated Sliced Array ={:?}", slice);
    let x = 28;
    let y = { 2 * x };
    println!("x from expression= {}", y);
    let tupule = ("Anurag", 21214, 56.87, false);
    println!("{:?}", tupule);
    struct Person {
        name: String,
        roll: u32,
        age: u8,
    }
    let anurag = Person {
        name: String::from("Anurag"),
        roll: 21214,
        age: 21,
    };
    let Person { name, roll, age } = anurag;
    println!("Details= {}, {}, {}", name, roll, age);
}

fn conditionals() {
    println!("Conditionals in rust!");
    let x: i32 = 100;
    if x < 0 {
        println!("X is a negative number")
    } else {
        if x % 10 == 0 {
            println!("X is positive and multiple of 10")
        } else {
            println!("X is a positive number")
        }
    }
}
fn iterations() {
    println!("Loops in rust!");
    let mut number: i32 = 1;
    loop {
        println!("Let's Get Rusty from loop");
        if number == 5 {
            break;
        }
        number += 1;
    }
    let mut number: i8 = 1;
    while number <= 5 {
        println!("Let's Get Rusty from while");
        number += 1;
    }
    for _i in 1..3 {
        println!("Lets Get Rusty from for loop");
    }
    let colors = ["red", "green", "blue"];
    for color in colors {
        println!("{}", color)
    }
    let string = "Hello World!";
    for char in string.chars() {
        print!("{}", char)
    }
    println!();
}

fn vectors() {
    println!("Vectors in Rust!");
    let v = vec![1, 2, 3, 4];
    let mut v1: Vec<i32> = Vec::new();
    println!("v= {:?}", v);
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);
    println!("v1= {:?}", v);
    v1.remove(2);
    v1.pop();
    println!("v1= {:?}", v);
    println!("{}", v1[0]);
    let x = v1.get(0);
    println!("{:?}", x)
}
fn strings() {
    let _word = String::from("Hello World");
    let mut word = String::new();
    word.push_str("Hello");
    word.push_str("World");
    let word1 = word; // now word is gone and cannot be used
    let word = word1.clone();
    println!("word1= {} word= {}", word1, word);
    for char in word1.chars() {
        print!("{}", char)
    }
}
fn hashmap() {
    println!();
    println!("Hashmap in Rust");
    let mut info: HashMap<i32, String> = HashMap::new();
    info.insert(1, String::from("One"));
    info.insert(2, String::from("Two"));
    info.insert(3, String::from("Three"));
    println!("{:?}", info.get(&1));
    println!("{:?}", info.remove(&2));
    info.insert(2, String::from("Two"));
    println!("{:?}", info);
    let size = info.len();
    println!("Size= {}", size);
    if info.contains_key(&4) {
        println!("Exist");
    } else {
        println!("Does not Exist");
    }
    for (key, val) in info.iter() {
        println!("{} {}", key, val);
    }
    let clonedmap = info.clone();
    println!("{:?}", clonedmap);
    let values = info.values();
    let keys = info.keys();
    println!("{:?} {:?}", keys, values);
}

fn set_operations() {
    println!("SET OPERATIONS : ");
    let s1 = HashSet::from(["red", "green", "blue"]);
    let s2 = HashSet::from(["red", "yellow", "black"]);
    let union: HashSet<_> = s1.union(&s2).collect();
    let intersection: HashSet<_> = s1.intersection(&s2).collect();
    let difference: HashSet<_> = s1.difference(&s2).collect();
    let symmetricdifference: HashSet<_> = s1.symmetric_difference(&s2).collect();
    println!("Set 1 = {:?}", s1);
    println!("Set 2 = {:?}", s2);
    println!("Union = {:?}", union);
    println!("Intersection = {:?}", intersection);
    println!("Difference = {:?}", difference);
    println!("Symmetric Difference = {:?}", symmetricdifference);
}
fn hashset() {
    let mut colors = HashSet::new();
    colors.insert("Red");
    colors.insert("Green");
    colors.insert("Blue");
    colors.insert("Red");
    println!("{:?}", colors);
    if colors.contains("Yellow") {
        println!("Exist");
    } else {
        println!("Does not Exist");
    }
    colors.remove("Red");
    println!("{:?}", colors);
    let size = colors.len();
    println!("Size= {}", size);
    if colors.is_empty() {
        println!("Empty Set");
    } else {
        println!("Not Empty Set");
    }
    for color in colors {
        print!("{} ,", color);
    }
    println!();
    set_operations();
}
fn main() {
    println!("Let's get Rusty");
    println!("Enter your choice:");
    println!("Enter 1 to continue:");
    println!("Enter 0 to exit:");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a number");
    if choice == 1 {
        data_types();
        conditionals();
        iterations();
        vectors();
        strings();
        hashmap();
        hashset();
    } else {
        println!("Terminated Program");
        panic!();
    }
}
