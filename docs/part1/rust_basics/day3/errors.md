# Errors handling
{:no_toc}

* TOC
{:toc}

There are no exceptions in the Rust language. Panic macro is for unrecoverable errors. Result type is core building block to return ok or err for recoverable errors. Its seems to be good language designers decision.
Libraries should focus on producing meaningful, structured error types/variants. Applications mainly consume errors.
As application developers we focus on consume errors.

# Panic
Application can raise unrecoverable error using panic macro. Application after panic will close with backtrace (in Java its named stacktrace). 
```rust
panic!("Panic message");
```
Application can not handle panic.

# core::result::Result
The Result<T, E> type is an enum that has two variants - Ok(T) for successful value or Err(E) for error value
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Return Result from function
Lets start to think about users and function than find it in db (or another store).

```rust
#[derive(Debug)]
pub struct User {
    id: u64,
    name: String
} 

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
```
Suppose we have three cases
1. User exists and function returns it
2. User doesn't exists
3. Error occurred in searching procedure

First two cases covered by Option<User>.
For third case declare struct
```rust
pub struct FindUserError {
    pub message: String,
}
```

find_user signature will be 
```rust
pub fn find_user(uid: u64) -> Result<Option<User>, FindUserError> {
  ...
}
```

And naive implementation 
```rust
if uid < 10 {
        Err(FindUserError {message: format!("Find user error. User id {}", uid)})
    } else if uid >= 10 && uid < 20 {
        let uname = format!("user name {}", uid);
        let user = User {id: uid, name: uname};
        Ok(Some(user))
    } else {
        Ok(None)
    }
```
Err(e) - return error e, Ok(v) - return result v. 

## Handle Result
### Terminate application
The idea is to covert application error to nonrecoverable one. This is not good way for server side applications. 
 
Result has methods unwrap and expect. Unwrap return a value if no error and terminate application with panic if error returned.
```rust
find_user(1).unwrap();
```
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: FindUserError { message: "Find user error. User id 1" }', day3_3_errors/src/main.rs:13:30
stack backtrace:
   0: rust_begin_unwind
....
```
Expect method has argument msg for custom error message
```rust
find_user(1).expect("user search fault");
```
```
thread 'main' panicked at 'user search fault: FindUserError { message: "Find user error. User id 1" }', day3_3_errors/src/main.rs:13:30
```
### Fallback value
If possible to treat UserFoundError as a user not found use unwrap_or_else for mapping error on a returned result
```rust 
    let fallback_user = find_user(1).unwrap_or_else(|_| None);
    println!("{:#?}", fallback_user); // falback_user is None
```

### React to the error
match could be used to react to error
```rust
match find_user(1) {
  Ok(u) => println!("user1 was found:  {:#?}", u),
  Err(e) => println!("user1 not found. The error is {:#?}", e),
};
```
Or if and even if let
```rust
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

    if let Ok(Some(u)) = find_user(12) {
        println!("{:#?} was found", u);
    }

```

### Wrap and throws up
The idea is to pass error handling to calling function. Optionally convert to more general error. Lets try to implement find_user2 function as a demo of this approach
```rust
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
``` 

## Map Result values and errors
Try to rewrite find_user2 function in more idiomatic way. 

The first step is convert find_user error from FindUserError to CommonError using Result map_err method
```rust
fn find_user3(uid: u64) -> Result<Option<User>, CommonError> {
  find_user(uid)
  .map_err(|e| CommonError {message: e.message})
}
```
Ok, the next step is handle None to CommonError transformation case. 

Function return type changes to Result<User, CommonError>. 

Then add ? after map_err call. ? is propagating error operator. Its immediately return error from function.

And next ok_or call. Its a Option method that return value if value is Some and Err if None. 
```rust
fn find_user3(uid: u64) -> Result<User, CommonError> {
  find_user(uid)
  .map_err(|e| CommonError {message: e.message})?
  .ok_or(CommonError {message: String::from("User not found")})
}
```
Final step is to implements access denied on id == 11. Use and_then method of Option.
```rust
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
```

# std::error::Error
In previous error wrapping example we really dont wrap error, but create more common one.

Rust have trait std::error::Error for wrapping errors and make chain of errors. Structs that implements Error should implements Display too.
Lets start to rewrite previous example. FindUserError now should implements Display and Error traits.
```rust
use std::{error::Error, fmt::Display};
use crate::common::User;

#[derive(Debug)]
pub struct FindUserError {
    pub message: String,
}

impl Display for FindUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FindUserError(message: {})", self.message)    
    }
}

impl Error for FindUserError {}

```
Now we can repeat function find_user, but with name e_find_user. Rename was made only for easy using both find_user and e_find_user in the same main.rs file.
```rust
pub fn e_find_user(uid: u64) -> Result<Option<User>, FindUserError> {
    if uid < 10 {
        Err(FindUserError {message: format!("Find user error. User id {}", uid)})
    } else if (10..20).contains(&uid) {
        let uname = format!("user name {}", uid);
        let user = User::new(uid, &uname[..]);
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
``` 

Now we declare CommonError as a enum.
```rust
#[derive(Debug)]
pub enum CommonError {
    Find(FindUserError),
    UserNotExists(String),
    AccessDenied
}


impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::Find(ref e) => write!(f, "{}", e),
            CommonError::UserNotExists(s) => write!(f, "{}", s),
            CommonError::AccessDenied => write!(f, "Access denied"),
        }
    }
}
```
Error trait has really useful method for wrapping another errors - source and we can implemented it for CommonError as below
```rust
impl Error for CommonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommonError::Find(ref e) => Some(e),
            CommonError::UserNotExists(_) => None,
            CommonError::AccessDenied => None
        }
    }
}
```
For the CommonError::Find it returns FindUserError wrapped instance.

```rust
pub fn e_find_user2(uid: u64) -> Result<User, CommonError> {
    e_find_user(uid)
    .map_err(CommonError::Find)?
    .ok_or_else(|| CommonError::UserNotExists(format!("User with id {} not found", uid)))
    .and_then(|user| 
        if user.id() == 11 {
            Err(CommonError::AccessDenied)
        } else {
            Ok(user)
        }  
    )   
 }
...

fn main() {
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
```
This code prints
```
FindUserError(message: Find user error. User id 1). Source: Some(FindUserError { message: "Find user error. User id 1" })
Access denied. Source: None
User with id 20 not found. Source: None
```

# Summary
Error handling approach in Rust is differs from Java and seems to like to functional Scala programming. Any struct can be used as error value, but its good to implement Error trait and source for error wrapping.
---
[<< Prev](./generics.md) &ensp; [Up](../index.md) &ensp; [Next >>]()
