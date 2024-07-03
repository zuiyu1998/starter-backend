use crate::Result;

pub trait TagIndexRepo {
    fn store_index(&mut self, tag: &str, id: i64) -> Result<()>;

    fn get_indexs(&self, tag: &str) -> Result<Vec<i64>>;
}
