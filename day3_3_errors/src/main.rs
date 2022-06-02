mod consume;
mod error_type;

use std::error::Error;

use consume::*;
use error_type::*;

fn main() {
    // Get result
    let res_user1 = find_user(1);
    let res_user2 = find_user(10);
    let res_user3 = find_user(30);
    println!("{:#?}", res_user1);
    println!("{:#?}", res_user2);
    println!("{:#?}", res_user3);

    // Fallback to None
    let fallback_user = find_user(1).unwrap_or(None);
    println!("{:#?}", fallback_user);
    
    // React to the error
    match find_user(1) {
        Ok(u) => println!("user1 was found:  {:#?}", u),
        Err(e) => println!("user1 not found. The error is {:#?}", e),
    };

    if find_user(2).is_err() {
        println!("Error");
    }

    if let Err(x) = find_user(2) {
        println!("Error {:#?}", x);
    }

    let u11 = find_user(11);
    if u11.is_ok() && u11.ok().is_some() {
        println!("User was found");
    }

    if let Ok(Some(usr)) = find_user(11) {
        println!("{:#?} was found", usr.name());
    }

    // Wrap and bubble inside find_user2
    println!("{:#?}", find_user2(11));

    // Map errors and values
    println!("{:#?}", find_user3(11));

    if let Err(err) = e_find_user2(1) {
        println!("{}. Source: {:?}", err, err.source());
    }

    if let Err(err) = e_find_user2(11){
        println!("{}. Source: {:?}", err, err.source());
    }

    if let Err(err) = e_find_user2(20) { 
        println!("{}. Source: {:?}", err, err.source());
    }
 }