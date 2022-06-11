use std::collections::HashMap;
use crate::object::Comic;
use faiss::{IdMap, FlatIndex, Index, Idx};

struct Database {
    comics: HashMap<i32, Comic>,
    index: IdMap<FlatIndex>
}

impl Database{
    pub fn new(dim: u32) -> Self {
        let index = FlatIndex::new_l2(dim).unwrap();
        let index = IdMap::new(index).unwrap();

        let comics = HashMap::new();
        Self{comics, index}
    }

    pub fn insert(&mut self, comic: Comic, vector: Vec<f32>){
        self.index.add_with_ids(&vector, &[Idx::new(comic.id as u64)])?;
        self.comics.insert(comic.id, comic);
    }
}