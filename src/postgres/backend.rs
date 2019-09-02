use crate::backend::Backend;

#[derive(Debug)]
pub struct Postgres;

impl Backend for Postgres {
    type QueryParameters = super::PostgresQueryParameters;
    type RawConnection = super::PostgresRawConnection;
    type Row = super::PostgresRow;
}

impl_from_sql_row_tuples_for_backend!(Postgres);
impl_into_query_parameters_for_backend!(Postgres);