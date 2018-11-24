///Struct to insert people
#[derive(Deserialize)]
pub struct CreatePerson {
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool
}

///Struct to find person
pub struct GetPerson {
    pub name: String,
}
