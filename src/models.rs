use super::schema::posts;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name=posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
