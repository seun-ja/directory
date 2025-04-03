use std::collections::BTreeMap;

use crate::{
    db::{
        TeacherSchema,
        school_directory_d::{SchoolDirectory, SchoolDirectoryDb},
        teachers_d::TeacherDb,
    },
    error::ApiError,
};
use sqlx::PgPool;

pub trait Searchable {
    fn search_by_term(data: Vec<Self>, term: &str) -> Result<BTreeMap<String, Self>, ApiError>
    where
        Self: Sized + SearchableTerm,
    {
        Ok(data
            .into_iter()
            .filter(|d| d.search_term() == term)
            .map(|d| (d.search_term(), d))
            .collect::<BTreeMap<String, Self>>())
    }
}

pub trait SearchableTerm {
    fn search_term(&self) -> String;
}

async fn _school_directory_cache(pool: PgPool) -> Result<SchoolDirectory, ApiError> {
    SchoolDirectoryDb { pool }.school_data().await
}

async fn _teachers_cache(pool: PgPool) -> Result<Vec<TeacherSchema>, ApiError> {
    TeacherDb { pool }.teacher_data().await
}

#[cfg(test)]
mod test {

    #[test]
    fn duplication_possibility_btree() {
        let mut new_tree: Vec<(&str, &str)> = Vec::new();
        new_tree.push(("John", "Doe"));
        new_tree.push(("John", "Cena"));

        assert_eq!(new_tree.len(), 2)
    }
}
