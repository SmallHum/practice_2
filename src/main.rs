use std::collections::BTreeMap;
use std::fmt::{Debug, Result, Formatter};

enum Role {
    NORMIE,
    ADMIN,
}

impl Role {
    fn value(&self) -> String {
        match *self {
            Role::NORMIE => "NORMIE".to_string(),
            Role::ADMIN => "ADMIN".to_string(),
        }
    }
}

struct User {
    name: String,
    email: String,
    role: Role,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let _ = write!(f, "{} ", self.role.value());
        let _ = write!(f, "{}", self.name);
        return Ok(());
    }
}

struct UserManager {
    // емейл ключ. зачем нам 2 пользователя с одинаковым емейлом
    // тем более так проще искать по емейлу
    users: BTreeMap<String, User>,
}

impl Debug for UserManager {
    // {:?} это дебаг отображение.
    // короче у коллекций есть только дебаг
    // кисс, вместо того чтобы фор и ин
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.users)
    }
}

impl UserManager {
    fn new() -> Self {
        Self {
            users: BTreeMap::new(),
        }
    }

    #[allow(nonstandard_style)]
    fn addUser(&mut self, user: User) {
        if self.users.get(&user.email).is_none(){
            println!("USER {:?} ADDED TO THE BASE", &user);
            self.users.insert(user.email.clone(), user);
        }
        else{
            println!("EMAIL {} IS ALREADY OCCUPIED", user.email);
        }
    }

    #[allow(nonstandard_style)]
    fn removeUserByEmail(&mut self, email: String) {
        // по тз не сказано что можно искать, поэтому пока оставим поиск тут
        let deleted_user: Option<User> = self.users.remove(&email);

        if deleted_user.is_none() {
            println!("USER WITH EMAIL {email} DOESNT EXIST IN THE BASE");
        }
        else {
            println!("{:?} REMOVED FROM THE BASE", deleted_user.unwrap());
        }
    }

    fn print(&self){
        println!("{:?}", self);
    }
}

fn main() {
    let mut manager = UserManager::new();

    manager.addUser(
        User{
            name: "ПЕТЯ228".to_string(),
            email: "irakirankakayaraznitsa@mail.ru".to_string(), 
            role: Role::NORMIE,
        }
    );

    manager.addUser(
        User{
            name: "ЛОЛОЛОШКА".to_string(),
            email: "romalololoshka@mail.ru".to_string(), 
            role: Role::ADMIN,
        }
    );

    manager.addUser(
        User{
            name: "ГЕНИАЛЬНЫЙ СЫЩИК".to_string(),
            email: "romalololoshka@mail.ru".to_string(), 
            role: Role::ADMIN,
        }
    );

    manager.print();

    manager.removeUserByEmail("irakirankakayaraznitsa@mail.ru".to_string());

    manager.print();

    
}
