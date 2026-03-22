
use actix_web::Responder;

#[derive(Clone, Debug, Eq, PartialEq)]
struct User {
    username: String,
    password: Option<String>,
    age: u8,
    sexe: Sexe,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "undefined".to_string(),
            password: Option::None,
            age: 1,
            sexe: Default::default(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Sexe {
    Male,
    Female,
    Other,
}

impl Default for Sexe {
    fn default() -> Self {
        Self::Male
    }
}

const USERS: Vec<User> = Vec::new();

async fn get_users() -> impl Responder {
    USERS
}
