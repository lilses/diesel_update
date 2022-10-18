use std::env;
use std::error::Error;
use diesel;
use diesel::dsl::Limit;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::{AsChangeset, Column, dsl, Expression, ExpressionMethods, Identifiable, Table};
use diesel::backend::Backend;
use diesel::expression::AsExpression;
use diesel::query_builder::IntoUpdateTarget;
use diesel::query_dsl::limit_dsl::LimitDsl;
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::{Bool, Integer};
use diesel_async;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolableConnection};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::deadpool::Pool;
use serde::*;
use update_diesel::schema;


#[derive(
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
    diesel::AsChangeset,
    Debug,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    Clone,
)]
#[diesel(table_name =schema::table_1)]
pub struct QTable1 {
    pub id: i32,
    pub confirm1: bool,
    pub confirm2: bool,
}

#[derive(diesel::Insertable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = schema::table_1)]
pub struct ITable1 {
    pub confirm1: bool,
    pub confirm2: bool,
}

#[derive(
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
    diesel::AsChangeset,
    Debug,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    Clone,
)]
#[diesel(table_name = schema::table_2)]
pub struct QTable2 {
    pub id: i32,
    pub confirm1: bool,
    pub confirm2: bool,
}

#[derive(diesel::Insertable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = schema::table_2)]
pub struct ITable2 {
    pub confirm1: bool,
    pub confirm2: bool,
}


#[tokio::main]
async fn main() -> Result<(),std::io::Error>{

    dotenv::dotenv();
    let manager =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            env::var("DATABASE_URL").unwrap(),
        );

    let pool =
        Pool::<AsyncPgConnection>::builder(manager)
            .build()
            .unwrap();

    let mut conn = pool.get().await.unwrap();

    let table_1_res = RunQueryDsl::get_results::<QTable1>(
        schema::table_1::dsl::table_1.filter(schema::table_1::dsl::confirm1.eq(false)),
        &mut conn,
    )
        .await
        .unwrap();

    let table_2_res = RunQueryDsl::get_results::<QTable1>(
        schema::table_1::dsl::table_1.filter(schema::table_1::dsl::confirm1.eq(false)),
        &mut conn,
    )
        .await
        .unwrap();


    // also does not compile
    let upd = update_confirm(
        schema::table_1::table,
        schema::table_1::dsl::id,
        schema::table_1::dsl::confirm1,
        1,
        &mut conn
    ).await;




    println!("Hello, world!");
    Ok(())
}

pub async fn update_confirm<'a, T, C1, C2, A1, A2, R, Conn, Q>(
    table: T,
    id_column: C1,
    bool_column: C2,
    id: A1,
    mut conn: Conn,
) -> Result<(), std::io::Error>
    where
        T: Table + Identifiable,
        T: FilterDsl<dsl::Eq<C1, A1>, Output = Limit<T>>,
        T: LimitDsl,
        C1: Column + ExpressionMethods + Expression<SqlType = Integer>,
        C2: Column + ExpressionMethods + Expression<SqlType = Bool> + AsChangeset,
        A1: Expression<SqlType = Integer> + AsExpression<Integer>,
        A2: Expression<SqlType = Bool> + AsExpression<Bool> + IntoUpdateTarget + AsChangeset,
        R: Send,
        Conn: AsyncConnection + PoolableConnection + 'a + Backend,
        Limit<T>: LoadQuery<'a, Conn, R>,
        <T as LimitDsl>::Output: 'a,
        <T as LimitDsl>::Output: IntoUpdateTarget + AsChangeset<Target = T>,
        // Update<T, T>: LimitDsl,
{
    // compiles
    let a = diesel::update(table.filter(id_column.eq(id)));

    // compiles
    let c = bool_column.eq(true);

    // does not compile
    // let d = a.set(c);

    // Ok(RunQueryDsl::get_result::<QWallet>(
    //     diesel::update(table.filter(column.eq(id))).set(column2.eq(true)),
    //     &mut conn,
    // )
    // .await
    // .map_err(|e| MyError::GeneralError(e.to_string()))?)
    Ok(())
}
