struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count: u64,
}

fn main(){
    let user1 = User {
        active : true,
        username : String::from("username"),
        email : String::from("email@gmail.com"),
        sign_in_count : 1
    };

    print!("user 1 username is : {}",user1.username);
}
