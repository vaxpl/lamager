use rocket::FromForm;

#[derive(Debug, FromForm)]
pub struct Person {
    pub uid: String,
    pub cn: String,
    pub mail: String,
    pub location: String,
}
