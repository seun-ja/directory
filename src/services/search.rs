use std::collections::BTreeMap;

use crate::error::ApiError;
use serde::Serialize;

pub trait Seachable {
    fn search_by_term<T: Serialize>(
        _datas: Vec<T>,
        _term: &str,
    ) -> Result<BTreeMap<&str, T>, ApiError> {
        todo!()
    }
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
