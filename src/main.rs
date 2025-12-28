use std::io::{self, Write};

struct User {
    email: String,
    password: String,
    is_admin: bool,
}

struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { 
            users: Vec::new(),
        }
    }

    fn get_user(&self, email: &str) -> Option<&User> {
        for user in &self.users {
            if user.email == email {
                return Some(user);
            }
        }
        None
    }

    fn get_user_mut(&mut self, email: &str) -> Option<&mut User> {
        for user in &mut self.users {
            if user.email == email {
                return Some(user);
            }
        }
        None
    }

    fn add_user(&mut self, email: String, password: String) -> String {
        if self.get_user(&email).is_some() {
            return String::from("user already exist")
        }

        let new_user = User {
            email,
            password,
            is_admin: false,
        };
        self.users.push(new_user);

        return String::from("created")
    }

    fn make_admin(&mut self, email: &str) {
        if let Some(user) = self.get_user_mut(email) {
            user.is_admin = true;
        } else {

        }
    }

    fn login(&self, email: &str, password: &str) -> bool {
        if let Some(user) = self.get_user(email) {
            if user.password == password {
                return true;
            }
        }
        
        false
    }
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
    io::stdout().flush().unwrap();
}

fn main() {
    let admin_email = String::from("admin@admin.com");
    let admin_password = String::from("admin");

    let mut db = Database::new();

    db.add_user(admin_email.clone(), admin_password.clone());
    db.make_admin(&admin_email);    

    let test_login = db.login(&admin_email, &admin_password);

    println!("{}", test_login);


    println!("Hello! This is register/login example");

}