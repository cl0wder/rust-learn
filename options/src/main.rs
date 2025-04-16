fn main() {
    let username = get_username(1);
    // match username {
    //     Some(name) => println!("{name}"),
    //     None => {}
    // }
    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    // get username from database
    let db_result = String::from("user name");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}

/*

struct User {
    id: u32,
    name: String,
}

fn get_user_name(id: u32) -> Option<String> {
    let database = [
        User {id: 1, name: String::from("Alice")},
        User {id: 2, name: String::from("Bob")},
        User {id: 3, name: String::from("Cindy")}
    ];
    for user in database {
        if user.id == id {
            return Some(user.name)
        }
    }
    None
}

fn main() {
    let user_id = 3;
    if let Some(name) = get_user_name(user_id) {
        println!("User's name: {name}");
    }
}

*/