use std::path::Path;

pub use serde::de::Deserialize;
pub use serde::de::DeserializeOwned;

pub fn deserialise_file<T, P>(path: P) -> T
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let data = std::fs::read_to_string(path).unwrap();
    let item = serde_json::from_str::<T>(&data).unwrap();
    item
}

pub fn deserialise_dir<T, P>(path: P) -> Vec<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let mut items = Vec::new();

    let read_dir = std::fs::read_dir(path).unwrap();
    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        let data = std::fs::read_to_string(path).unwrap();
        let item = serde_json::from_str::<T>(&data).unwrap();
        items.push(item);
    }

    items
}
