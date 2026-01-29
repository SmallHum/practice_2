use std::collections::BTreeMap;
use std::fmt::{Debug, Result, Formatter};

enum Role {
    NORMIE,
    ADMIN,
}

// кисс и ъягни. просто скажу что в данном коде структ String не понадобится

impl Role {
    fn value(&self) -> &'static str {
        match *self {
            Role::NORMIE => "NORMIE",
            Role::ADMIN => "ADMIN",
        }
    }
}

struct User {
    name: &'static str,
    email: &'static str,
    role: Role,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.role.value(), self.name)
    }
}

struct UserManager {
    // емейл ключ. зачем нам 2 пользователя с одинаковым емейлом
    // тем более так проще искать по емейлу
    users: BTreeMap<&'static str, User>,
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
        if self.users.get(user.email).is_none(){
            println!("USER {:?} ADDED TO THE BASE", &user);
            self.users.insert(user.email, user);
        }
        else{
            println!("EMAIL {} IS ALREADY OCCUPIED", user.email);
        }
    }

    #[allow(nonstandard_style)]
    fn removeUserByEmail(&mut self, email: &'static str) {
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

    // добавляем  нормиса
    manager.addUser(
        User{
            name: "ПЕТЯ228",
            email: "irakirankakayaraznitsa@mail.ru", 
            role: Role::NORMIE,
        }
    );

    // добавялем админа
    manager.addUser(
        User{
            name: "ЛОЛОЛОШКА",
            email: "romalololoshka@mail.ru", 
            role: Role::ADMIN,
        }
    );

    // О НЕТ ЛОЛОЛОШКУ ХОЧЕТ ВЗЛОМАТЬ ГЕНИАЛЬНЫЙ СЫЩИК ЮТУБ КАНАЛОВ
    // но у него это не получится т.к система запретит
    manager.addUser(
        User{
            name: "ГЕНИАЛЬНЫЙ СЫЩИК",
            email: "romalololoshka@mail.ru", 
            role: Role::ADMIN,
        }
    );

    // отобразить
    manager.print();

    // удалить реального пользователя
    manager.removeUserByEmail("irakirankakayaraznitsa@mail.ru");
    
    // удалить несуществующего пользователя
    manager.removeUserByEmail("wingdingaster@gmail.com");

    manager.print();

    
}
