use std::collections::hash_map;
use std::slice;
use serde::{de::DeserializeOwned};

use crate::serialization::deserialize_data;

/// Iterator object for iterating over keys and values in PickleDB. Returned in [PickleDb::iter()](struct.PickleDb.html#method.iter)
pub struct PickleDbIterator<'a> {
    pub(crate) map_iter: hash_map::Iter<'a, String, String>
}

impl<'a> Iterator for PickleDbIterator<'a> {
    type Item = PickleDbIteratorItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.map_iter.next() {
            Some((key, value_as_string)) => Some(PickleDbIteratorItem { key: key, value_as_string: value_as_string}),
            None => None
        }
    }
}

/// The object returned in each iteration when iterating over keys and values in PickleDB
pub struct PickleDbIteratorItem<'a> {
    key: &'a str,
    value_as_string: &'a str,
}

impl<'a> PickleDbIteratorItem<'a> {

    /// Get the key
    pub fn get_key(&self) -> &str {
        self.key
    }

    /// Get the value of the key.
    /// 
    /// The key is always a string but the value can be of any type. It's the user's
    /// responsibility to know the value type and give it while calling this method.
    /// If the key doesn't exist or if the type is wrong, `None` will be returned.
    /// Otherwise `Some(V)` will be returned.
    /// Since the values are stored in a serialized way the returned object is
    /// not a reference to the value stored in a DB but actually a new instance of it.
    /// The method returns `Some(V)` if deserialization succeeds or `None` otherwise.
    /// 
    pub fn get_value<V>(&self) -> Option<V> where V: DeserializeOwned {
        deserialize_data::<V>(self.value_as_string)
    }
}

/// Iterator object for iterating over items in a PickleDB list. Returned in [PickleDb::liter()](struct.PickleDb.html#method.liter)
pub struct PickleDbListIterator<'a> {
    pub(crate) list_iter: slice::Iter<'a, String>
}

impl<'a> Iterator for PickleDbListIterator<'a> {
    type Item = PickleDbListIteratorItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list_iter.next() {
            Some(value_as_string) => Some(PickleDbListIteratorItem { value_as_string: value_as_string }),
            None => None
        }
    }
}

/// The object returned in each iteration when iterating over PickleDB list
pub struct PickleDbListIteratorItem<'a> {
    value_as_string: &'a str,
}

impl<'a> PickleDbListIteratorItem<'a> {

    /// Get the item in the current position.
    /// 
    /// This method retrieves the item in the current position. It's the user's responsibility 
    /// to know what is the correct type of the item and give it while calling this method.
    /// Since the item in the lists are stored in a serialized way the returned object 
    /// is not a reference to the item stored in a DB but actually a new instance of it.
    /// The method returns `Some(V)` if deserialization succeeds or `None` otherwise.
    /// 
    pub fn get_item<V>(&self) -> Option<V> where V: DeserializeOwned {
        deserialize_data(self.value_as_string)
    }
}