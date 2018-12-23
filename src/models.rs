use super::schema::people;

#[derive(Serialize, Queryable, Debug, GraphQLObject)]
pub struct Person {
    pub id: String,
    #[graphql(description="Name of the person")]
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}


#[derive(Insertable)]
#[table_name = "people"]
pub struct NewPerson{
    pub id: String,
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}

#[derive(GraphQLInputObject)]
#[graphql(description="New person object")]
pub struct NewPersonGraph{
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool,
}