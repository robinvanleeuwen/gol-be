use super::schema::runcount;

#[derive(Queryable)]
pub struct RunCount {
    pub id: i32,
    pub run: Option<i32>,
    pub count: Option<i32>,
}

#[derive(Insertable)]
#[table_name="runcount"]
pub struct NewRunCount<'a> {
    pub run: &'a i32,
    pub count: &'a i32,
}