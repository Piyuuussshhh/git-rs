pub trait ObjectSerialize {
    fn serialize(&self, path: &str) -> String;
}

pub trait ObjectDeserialize<T> {
    fn deserialize(content: &str) -> T;
}