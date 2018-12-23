use super::schema::people;

#[derive(Serialize, Queryable, Debug, GraphQLObject)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}


#[derive(Insertable, GraphQLInputObject)]
#[table_name = "people"]
pub struct NewPerson{
    pub id: String,
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}
