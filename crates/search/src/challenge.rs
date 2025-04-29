use tantivy::{
    Index,
    schema::{FAST, INDEXED, STORED, STRING, Schema, TEXT},
};
use tokio::sync::OnceCell;

static CHALLENGE: OnceCell<(Index, Schema)> = OnceCell::const_new();

pub async fn instance() -> &'static (Index, Schema) {
    CHALLENGE
        .get_or_init(|| async {
            let mut schema_builder = Schema::builder();

            schema_builder.add_text_field("id", STRING | STORED);
            schema_builder.add_text_field("title", STRING | STORED);
            schema_builder.add_text_field("description", TEXT | STORED);
            schema_builder.add_i64_field("category", INDEXED | STORED);
            schema_builder.add_json_field("tags", TEXT | STORED);
            schema_builder.add_i64_field("created_at", FAST | STORED);

            let schema = schema_builder.build();

            let index = Index::create_in_ram(schema.clone());

            (index, schema)
        })
        .await
}
