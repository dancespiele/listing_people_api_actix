#![warn(proc_macro_derive_resolution_fallback)]
use super::schema::people;

#[derive(Serialize, Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}


#[derive(Insertable)]
#[table_name = "people"]
pub struct NewPerson<'a>{
    pub id: i32,
    pub name: &'a str,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}