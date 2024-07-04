use crate::Result;

pub trait TagIndexRepo {
    fn store_index(&mut self, tag: &str, id: i32) -> Result<()>;

    fn get_indexs(&self, tag: &str, page_size: i32, page: i32) -> Result<Vec<i32>>;
}
