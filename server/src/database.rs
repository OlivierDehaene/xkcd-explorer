use crate::object::{Comic, Similarity};
use anyhow::{Context, Result};
use faiss::{FlatIndex, IdMap, Idx, Index};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    comics: HashMap<u64, Comic>,
    index: IdMap<FlatIndex>,
}

impl Database {
    pub fn new(dim: u32) -> Self {
        let index = FlatIndex::new_l2(dim).unwrap();
        let index = IdMap::new(index).unwrap();

        let comics = HashMap::new();
        Self { comics, index }
    }

    pub fn exists(&self, id: &u64) -> bool {
        self.comics.contains_key(id)
    }

    pub fn get(&self, id: &u64) -> Option<&Comic> {
        self.comics.get(id)
    }

    pub fn insert(&mut self, comic: Comic, vector: Vec<f32>) -> Result<()> {
        if !self.exists(&comic.id) {
            self.index
                .add_with_ids(&vector, &[Idx::new(comic.id as u64)])
                .context("failed to add vector in the FAISS index")?;
            self.comics.insert(comic.id, comic);
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!(
                "id {} already exists",
                comic.id
            )))
        }
    }

    pub fn search(&mut self, vector: Vec<f32>, topk: usize) -> Result<Vec<Similarity>> {
        let results = self
            .index
            .search(&vector, topk)
            .context("failed to search vector in FAISS index")?;

        let similar_comics: Vec<Similarity> = results
            .distances
            .iter()
            .zip(results.labels.iter())
            .filter(|(_, label)| label.to_native() > 0)
            .map(|(distance, label)| Similarity {
                comic: Some(
                    self.comics
                        .get(&(label.to_native() as u64))
                        .unwrap()
                        .clone(),
                ),
                distance: *distance,
            })
            .collect();
        Ok(similar_comics)
    }
}
