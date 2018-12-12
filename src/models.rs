use super::schema::people;

#[derive(Serialize, Queryable, Debug)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}


#[derive(Insertable)]
#[table_name = "people"]
pub struct NewPerson<'a>{
    pub id: String,
    pub name: &'a str,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}