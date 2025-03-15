pub trait TomlData {
    fn get_data(id: u32) -> Option<Self>
    where
        Self: Sized;

    fn update_and_save<K, V>(id: u32, field: K, new_value: V) -> bool
    where
        Self: Sized,
        K: AsRef<str>,
        V: ToString;
    fn id(&self) -> u32;
    fn name(&self) -> String;
}
