use deadpool_diesel::postgres::{Object, Pool};
use crate::core::errors::error::AppError;
use crate::core::errors::error_handler::throw_error;

/// Attempts to retrieve a connection from the provided `Pool` (which should be set up on app launch).
///
/// This function interacts with the provided connection pool and tries to
/// obtain a connection asynchronously. If the connection is successful, it
/// returns `Some(connection)`. If the connection fails, it logs the error
/// using the custom `throw_error` function and returns `None`.
///
/// # Arguments
///
/// * `pool` - A `deadpool_postgres::Pool` that is used to get a database connection.
///
/// # Returns
///
/// * `Some(deadpool_postgres::Object)` - If a connection is successfully retrieved from the pool.
/// * `None` - If there is an error in getting the connection.
///
/// # Errors
///
/// The function logs any errors that occur during the process of getting a connection
/// using `throw_error` and does not propagate the error further. It simply returns `None`
/// in case of failure.
///
/// # Example
///
/// ```rust,no_run
/// # use deadpool_postgres::Pool;
/// # use crate::create_connection;
/// # #[tokio::main]
/// # async fn main() {
/// let pool: Pool = /* initialize your pool here */;
/// if let Some(conn) = create_connection(pool).await {
///     // Use the connection
///     println!("Connection successful!");
/// } else {
///     // Handle the case where the connection fails
///     println!("Failed to get connection.");
/// }
/// # }
/// ```
///
/// This function is useful for handling connection errors gracefully in asynchronous
/// contexts where obtaining a database connection may fail, but the program should not panic.
pub async fn create_connection(pool: &Pool) -> Option<Object> {
    match pool.get().await {
        Ok(conn) => Some(conn),
        Err(e) => {
            // Handle the error by logging or throwing it using our custom error handler
            throw_error(AppError::DatabaseConnectionError(e.to_string()));
            None
        }
    }
}