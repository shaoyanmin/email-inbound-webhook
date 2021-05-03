use sqlx::database::HasArguments;

pub type Query<'a> =
sqlx::query::Query<'a, sqlx::Sqlite, <sqlx::Sqlite as HasArguments<'a>>::Arguments>;

pub type QueryAs<'a, T> =
sqlx::query::QueryAs<'a, sqlx::Sqlite, T, <sqlx::Sqlite as HasArguments<'a>>::Arguments>;
