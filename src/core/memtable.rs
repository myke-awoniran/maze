/*
   memtable- A key value in memory data store. This is a temporary data storage, before being flushed to the disk
*/

pub struct MemTable {}

impl MemTable {
    fn put(key: String, value: String) {}

    fn get(key: String) -> Option<String> { None }
}
