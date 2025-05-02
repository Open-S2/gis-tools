use super::KVStore;
use crate::{
    U64,
    file::{FileOptions, S2FileStore},
};
use core::{
    cell::{RefCell, RefMut},
    ops::{Deref, DerefMut},
};
use s2json::Properties;
use serde::{Serialize, de::DeserializeOwned};

/// File based reader that implements the KVStore trait
#[derive(Debug)]
pub struct FileKV<K: U64 = u64, V: Serialize + DeserializeOwned + Clone = Properties> {
    store: RefCell<S2FileStore<K, V>>,
    tmp_val: RefCell<Option<V>>,
}
impl<K: U64, V: Serialize + DeserializeOwned + Clone> FileKV<K, V> {
    /// Builds a new File based KV with defined options
    pub fn new_with_options(file_name: Option<&str>, opts: Option<FileOptions>) -> FileKV<K, V> {
        FileKV { store: S2FileStore::new(file_name, opts).into(), tmp_val: None.into() }
    }
}
impl<K: U64, V: Serialize + DeserializeOwned + Clone> KVStore<K, V> for FileKV<K, V> {
    fn new(file_name: Option<&str>) -> FileKV<K, V> {
        FileKV { store: S2FileStore::new(file_name, None).into(), tmp_val: None.into() }
    }
    fn len(&self) -> u64 {
        self.store.borrow().len()
    }
    fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }
    fn get(&self, key: K) -> Option<&V> {
        let mut store = self.store.borrow_mut();
        if let Some(vec) = store.get(key, Some(1)) {
            if vec.1.is_empty() {
                return None;
            }
            self.tmp_val.borrow_mut().replace(vec.1[0].clone());
            if let Some(val_ref) = self.tmp_val.borrow().as_ref() {
                return Some(unsafe { &*(val_ref as *const V) });
            }
        }
        None
    }
    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let mut store = self.store.borrow_mut();
        if let Some(vec) = store.get(key, Some(1)) {
            if vec.1.is_empty() {
                return None;
            }
            self.tmp_val.borrow_mut().replace(vec.1[0].clone());
            if let Some(val_ref) = self.tmp_val.borrow_mut().as_mut() {
                return Some(unsafe { &mut *(val_ref as *mut V) });
            }
        }
        None
    }
    fn set(&mut self, key: K, value: V) {
        self.store.borrow_mut().set(key, value);
    }
    fn has(&self, key: K) -> bool {
        self.store.borrow_mut().has(key)
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        V: 'a,
    {
        KVStoreIterator { store: self.store.borrow_mut(), curr_kv: RefCell::new(None), index: 0 }
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a K, &'a mut V)>
    where
        V: 'a,
    {
        KVStoreMutIterator { store: self.store.borrow_mut(), curr_kv: RefCell::new(None), index: 0 }
    }
    fn cleanup(&mut self) {
        self.store.borrow_mut().cleanup();
    }
}
struct KVStoreIterator<'a, K: U64, V: Serialize + DeserializeOwned + Clone> {
    store: RefMut<'a, S2FileStore<K, V>>,
    curr_kv: RefCell<Option<(K, V)>>,
    index: u64,
}
impl<'a, K: U64, V: Serialize + DeserializeOwned + Clone> Iterator for KVStoreIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let next_kv = self.store.get_index(self.index);
        self.index += 1;

        if let Some(kv) = next_kv {
            *self.curr_kv.borrow_mut() = Some(kv);
            if let Some((k, v)) = self.curr_kv.borrow().deref() {
                unsafe {
                    let k_ptr: *const K = k;
                    let v_ptr: *const V = v;
                    Some((&(*k_ptr), &(*v_ptr)))
                }
            } else {
                None
            }
        } else {
            *self.curr_kv.borrow_mut() = None;
            None
        }
    }
}
struct KVStoreMutIterator<'a, K: U64, V: Serialize + DeserializeOwned + Clone> {
    store: RefMut<'a, S2FileStore<K, V>>,
    curr_kv: RefCell<Option<(K, V)>>,
    index: u64,
}
impl<'a, K: U64, V: Serialize + DeserializeOwned + Clone> Iterator
    for KVStoreMutIterator<'a, K, V>
{
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let next_kv = self.store.get_index(self.index);
        self.index += 1;

        if let Some(kv) = next_kv {
            *self.curr_kv.borrow_mut() = Some(kv);
            if let Some((k, v)) = self.curr_kv.borrow_mut().deref_mut() {
                unsafe {
                    let k_ptr: *const K = k;
                    let v_ptr: *mut V = v;
                    Some((&(*k_ptr), &mut (*v_ptr)))
                }
            } else {
                None
            }
        } else {
            *self.curr_kv.borrow_mut() = None;
            None
        }
    }
}

// struct DroppableV<V: Clone> {
//     value: V,
//     callback: Box<dyn FnOnce(V)>,
// }

// impl<V: Clone> Drop for DroppableV<V> {
//     fn drop(&mut self) {
//         (self.callback)(self.value.clone());
//     }
// }
