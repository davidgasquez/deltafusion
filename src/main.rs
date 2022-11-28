use datafusion::execution::context::SessionContext;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let ctx = SessionContext::new();
    let table = deltalake::open_table("delta_table").await.unwrap();

    // Register the table with the context
    if let Err(e) = ctx.register_table("delta_table", Arc::new(table)) {
        println!("Error registering table: {}", e);
    }

    // Run SQL query
    let df = ctx.sql("SELECT * FROM delta_table").await.unwrap();
    df.show().await.unwrap();
}
