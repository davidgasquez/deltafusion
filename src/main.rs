use datafusion::execution::context::SessionContext;
use deltalake::operations::DeltaOps;
use deltalake::DeltaTableBuilder;

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = SessionContext::new();

    let table = DeltaTableBuilder::from_uri("delta_table").load().await?;

    // let table = deltalake::open_table("delta_table").await.unwrap();

    // let m = table.get_metadata().unwrap();
    // print!("Table metadata: {:#?}", m);

    // Register the table with the context
    if let Err(e) = ctx.register_table("delta_table", Arc::new(table)) {
        println!("Error registering table: {}", e);
    }

    // Run SQL query
    let df = ctx.sql("SELECT * FROM delta_table").await.unwrap();
    df.show().await.unwrap();

    let batches = ctx
        .sql("SELECT * FROM delta_table")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    println!("Batches: {:#?}", batches);

    for batch in batches {
        let _table = DeltaOps(table).write(vec![batch.clone()]).await?;
    }

    Ok(())
}
