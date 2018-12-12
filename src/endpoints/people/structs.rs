///Struct to insert people
#[derive(Deserialize, Debug)]
pub struct CreatePerson {
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}

#[derive(Deserialize, Debug)]
pub struct People{ 
    pub list: Vec<CreatePerson>
}

///Struct to find person
#[derive(Deserialize)]
pub struct GetPerson {
    pub name: String,
}

///Struct to get all the people
pub struct AllPeople;

///Struct to delete person
#[derive(Deserialize)]
pub struct DeletePerson {
    pub name: String,
}