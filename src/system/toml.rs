pub trait TomlData {
    fn get_data(id: u8) -> Option<Self>
    where
        Self: Sized;

    fn update_and_save<K, V>(id: u8, field: K, new_value: V) -> bool
    where
        Self: Sized,
        K: AsRef<str>,
        V: ToString;
}
