use std::collections::HashMap;

fn vector_init_demo() {
    // Create empty mutable vector via new method
    let v1 = Vec::<u32>::new();
    println!("v1: {:?}", v1);
    
    // Create immutable vector via vec! macros
    let v2: Vec<u32> = vec!(1, 2, 3);
    println!("v2: {:?}", v2);
}

fn vector_as_stack_demo() {
    let mut v1 = Vec::<u32>::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);
    let x = v1.pop(); // Some(3)
    let y = v1.pop(); // Some(2)
    let z = v1.pop(); // Some(1)
    let o = v1.pop(); // None
    let len = v1.len(); // 3
    println!("x={:?}, y={:?}, z={:?}, o={:?}, len={:?}", x, y, z, o, len);
}

fn vector_as_list_demo() {
    let mut v1 = vec!(1, 2, 3, 4, 5);
    let x = v1.get(0);
    let y = v1[1];
    println!("x={:?}, y={:?}, v1.len = {}", x, y, v1.len());

    v1[0] = 10;
    println!("v1[0]={:?}, v1.len = {}", v1[0], v1.len());

    v1.remove(0);
    println!("v1={:?}", v1);

    v1.insert(0, 1);
    println!("v1={:?}", v1);
}

fn vector_search_demo() {
    let v1: Vec<u32> = vec!(1, 2, 3, 4, 5);
    let x = v1.contains(&3);
    println!("v1 contains 3? = {}", x);
    let p = v1.iter().position(|x| x > &2);
    println!("v1 position for x > 2 = {:?}", p);
}

fn vector_transform_demo() {
    let v1: Vec<u32> = vec!(1, 2, 3, 4, 5);
    
    let evens: Vec<u32> = v1.into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
    println!("evens = {:?}", evens);
    
    let multed: Vec<u32> = evens.into_iter()
        .map(|x| x * 10)
        .collect();

    println!("multed = {:?}", multed);    
}

fn hashmap_demo() {
    let mut users_map: HashMap<u32, String> = HashMap::new(); 
    users_map.insert(1, "Mike".to_string());
    users_map.insert(2, "Alex".to_string());
    users_map.insert(3, "Mary".to_string());

    println!("users_map {:?}", users_map);

    let maybe_mike = users_map.get(&1);
    println!("users_map.get(&1) = {:?}", &maybe_mike);

    let maybe_user10 = users_map.get(&10);
    println!("users_map.get(&10) = {:?}", &maybe_user10);

    let filtered: HashMap<u32, String> = users_map.into_iter()
        .filter(|(k, _v)| k > &1)
        .map(|(k, v)| (k + 1, format!("!={}=!", v) ))
        .collect();
    println!("filtered key > 1 = {:?}", filtered);
}

fn main() {
    vector_init_demo();
    vector_as_stack_demo();
    vector_as_list_demo();
    vector_search_demo();
    vector_transform_demo();

    hashmap_demo();
}
