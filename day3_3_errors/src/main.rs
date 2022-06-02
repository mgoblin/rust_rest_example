mod consume;

pub use consume::*;

fn main() {
    // Get result
    let res_user1 = find_user(1);
    let res_user2 = find_user(10);
    let res_user3 = find_user(30);
    println!("{:#?}", res_user1);
    println!("{:#?}", res_user2);
    println!("{:#?}", res_user3);

    // Fallback to None
    let fallback_user = find_user(1).unwrap_or_else(|_| None);
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

    if let Ok(Some(u)) = find_user(11) {
        println!("{:#?} was found", u);
    }

    // Wrap and bubble inside find_user2
    println!("{:#?}", find_user2(11));

    // Map errors and values
    print!("{:#?}", find_user3(11));

 }

 fn find_user3(uid: u64) -> Result<User, CommonError> {
    find_user(uid)
    .map_err(|e| CommonError {message: e.message})?
    .ok_or(CommonError {message: String::from("User not found")})
    .and_then(|user| 
        if user.id() == 11 {
            Err(CommonError { message: String::from("Access denied")})
        } else {
            Ok(user)
        }  
    )    
 }

 fn find_user2(uid: u64) -> Result<User, CommonError> {
    let r = find_user(uid);
    if let Ok(Some(user)) = r {
        if user.id() == 11 {
            let err = CommonError { message: String::from("Access denied")};
            Err(err) 
        } else {
            Ok(user)
        }
    } else if let Ok(None) = r {
        let err = CommonError { message: String::from("User not found")};
        Err(err)
    } else {
        let err = CommonError { message: r.unwrap_err().message};
        Err(err)
    }    
 }
