### Update Diesel

This repo demonstrates how to update
tables dynamically without boxing.

TLDR; Updating tables by passing
table names and columns.

---

Table 1 and Table 2 both share
same columns in database.

The goal is to have function
that updates any of those
by giving table name and 
table column as argument.

👀
Need to give function correct
trait bounds.

```rust
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
    
    Ok(())
}
```

