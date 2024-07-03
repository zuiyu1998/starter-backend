use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::tag::TagIndexRepo;
use crate::Result;
use abi::{config::Config, tokio::fs};
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Field, OwnedValue, Schema, Value, INDEXED, STORED, TEXT},
    DocAddress, Index, IndexWriter, ReloadPolicy, Score,
};

pub struct TantivyTag {
    index: Index,
    schema: Schema,
}

impl TantivyTag {
    pub async fn new(config: &Config) -> Result<Self> {
        let index_path = config.system.get_index_path();

        fs::create_dir_all(&index_path).await?;

        let tag = Self::path(index_path).await?;

        Ok(tag)
    }

    pub async fn path(directory_path: impl AsRef<Path>) -> Result<Self> {
        let dir = MmapDirectory::open(directory_path).unwrap();

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("tag", TEXT);
        schema_builder.add_i64_field("id", INDEXED | STORED);
        let schema = schema_builder.build();

        let index = Index::open_or_create(dir, schema.clone())?;

        Ok(TantivyTag { index, schema })
    }
}

impl TagIndexRepo for TantivyTag {
    fn store_index(&mut self, tag: &str, id: i64) -> Result<()> {
        let tag_field = self.schema.get_field("tag")?;
        let id_field = self.schema.get_field("id")?;

        let mut index_writer: IndexWriter = self.index.writer(100_000_000)?;

        let doc = doc!(
            tag_field => tag,
            id_field => id
        );
        index_writer.add_document(doc)?;

        index_writer.commit()?;

        Ok(())
    }

    fn get_indexs(&self, tag: &str) -> Result<Vec<i64>> {
        let tag_field = self.schema.get_field("tag")?;
        let id_field = self.schema.get_field("id")?;

        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(&self.index, vec![tag_field]);

        let query = query_parser.parse_query(tag)?;

        let collecter = TopDocs::with_limit(10).and_offset(0);

        let top_docs: Vec<(Score, DocAddress)> = searcher.search(&query, &collecter)?;

        let mut ids: HashSet<i64> = HashSet::default();

        for (_score, address) in top_docs.into_iter() {
            let doc: HashMap<Field, OwnedValue> = searcher.doc(address)?;

            if let Some(value) = doc.get(&id_field) {
                ids.insert(value.as_i64().unwrap());
            }
        }

        Ok(ids.into_iter().collect::<Vec<i64>>())
    }
}

mod test {
    use abi::tokio;

    #[tokio::test]
    async fn test_tag() {
        use super::TantivyTag;
        use crate::tag::TagIndexRepo;
        use tempfile::TempDir;

        let index_path = TempDir::new().unwrap();

        let mut tag_index = TantivyTag::path(index_path.path()).await.unwrap();

        tag_index.store_index("test", 1).unwrap();
        tag_index.store_index("test1", 2).unwrap();

        let ids = tag_index.get_indexs("test").unwrap();

        assert_eq!(ids.len(), 1);
    }
}
