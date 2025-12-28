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
        }
    }

    fn login(&self, email: &str, password: &str) -> Option<&User> {
        if let Some(user) = self.get_user(email) {
            if user.password == password {
                return Some(user);
            }
        }
        None
    }

    fn delete_user(&mut self, email: &str) {
        self.users.retain(|user| user.email != email);
    }

    fn change_password(&mut self, email: &str, password: String) {
        if let Some(user) = self.get_user_mut(email) {
            user.password = password;
        }
    }
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
    io::stdout().flush().unwrap();
}

fn get_input_int(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut shit = String::new();

        io::stdin()
            .read_line(&mut shit)
            .expect("Failed to read line");

        match shit.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter valid choise"),
        }
    }
}

fn get_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut shit = String::new();

        io::stdin()
            .read_line(&mut shit)
            .expect("Failed to read line");

        break shit.trim().to_string();
    }
}

fn main() {
    let admin_email = String::from("admin@admin.com");
    let admin_password = String::from("admin");

    let mut db = Database::new();

    db.add_user(admin_email.clone(), admin_password.clone());
    db.make_admin(&admin_email);    

    println!("Hello! This is register/login example");
    loop {
        println!("Menu: ");
        println!("1 - login");
        println!("2 - register");
        println!("");
        println!("0 - exit");

        let choise = get_input_int("Enter your choise: ");
        match choise {
            1 => {
                let login_email = get_input("Enter email: ");
                let login_password = get_input("Enter password: ");

                let login_user = db.login(&login_email, &login_password);
                if let Some(u) = login_user {
                    let user_id_admin = u.is_admin;
                    loop {
                        //get_input("");
                        //clear_screen();
                        println!("You logged in as: {}", login_email);
                        println!("");
                        println!("Menu: ");
                        if user_id_admin {
                            println!("0 - admin panel");
                        }
                        println!("1 - change password");
                        println!("");
                        println!("2 - logout");

                        let choise = get_input_int("Enter your choise: ");

                        match choise {
                            0 => {
                                if user_id_admin {
                                    loop {
                                        //get_input("");
                                        //clear_screen();
                                        println!("Hi admin!");
                                        println!("");
                                        println!("Menu: ");
                                        println!("1 - see all users");
                                        println!("2 - delete user");
                                        println!("");
                                        println!("0 - exit");

                                        let choise = get_input_int("Enter your choise: ");

                                        match choise {
                                            0 => break,
                                            1 => {
                                                clear_screen();
                                                let mut user_id = 0;
                                                println!("All user list: ");
                                                for user in &db.users {
                                                    user_id += 1;
                                                    println!("{user_id}. {}", user.email);
                                                }
                                            },
                                            2 => {
                                                let delete_user = get_input("Enter email to delete: ");
                                                db.delete_user(&delete_user);
                                            }
                                            _ => {
                                                continue;
                                            }
                                        }
                                    }

                                } else {
                                    println!("You dont have rights to do this");
                                    continue;
                                }
                            },
                            1 => {
                                let new_password = get_input("Enter your new password: ");
                                db.change_password(&login_email, new_password);
                            },
                            2 => {
                                break;
                            }
                            _ => {
                                continue;
                            },
                        }
                    }
                } else {
                    println!("Login failed!");
                    continue;
                }
            },
            2 => {
                let register_email = get_input("Enter email: ");
                let register_password = get_input("Enter password: ");
                
                let register_user = db.add_user(register_email, register_password);
                println!("{}", register_user);
            },
            0 => {
                clear_screen();
                println!("Goodbye!");
                break;
            }
            _ => {
                continue;
            }
        }
    }
}