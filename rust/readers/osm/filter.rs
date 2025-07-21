use super::{
    OSMReader,
    node::IntermediateNode,
    primitive::PrimitiveBlock,
    relation::IntermediateRelation,
    way::{IntermediateWay, WayNodes},
};
use crate::{data_store::KVStore, parsers::Reader};
use alloc::{collections::BTreeMap, string::String};
use s2json::{MValue, VectorPoint};

/// OSM Filterable is a trait to ensure that an object can be filtered
pub trait OSMFilterable {
    /// Check if the object is filterable
    fn is_filterable<
        T: Reader,
        _N: KVStore<u64, VectorPoint<MValue>>,
        N: KVStore<u64, IntermediateNode>,
        _W: KVStore<u64, WayNodes>,
        W: KVStore<u64, IntermediateWay>,
        R: KVStore<u64, IntermediateRelation>,
    >(
        &self,
        pb: &PrimitiveBlock,
        reader: &mut OSMReader<T, _N, N, _W, W, R>,
    ) -> bool;
}

/// Types of objects that can be filtered
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OSMTagFilterType {
    /// Apply filter to all object types
    All,
    /// Apply filter to nodes
    Node,
    /// Apply filter to ways
    Way,
    /// Apply filter to relations
    Relation,
}

/// Filter map. Used internally by TagFilter
type FilterMap = BTreeMap<String, Option<String>>;

/// # Tag Filter
///
/// ## Description
/// Builds a filter for the tags when parsing data.
/// Can parse tags from nodes, ways and relations.
/// Also allows the ability to add tags that apply to all object types.
/// Can filter by key, but also both key and value.
///
/// ## Usage
///
/// Note that if you don't add a filter for a specific type, then that type will pass all
/// key-value pairs through.
///
/// ```rs
/// use gistools::readers::{OSMTagFilter, OSMTagFilterType};
///
/// const filter = OSMTagFilter::default();
/// // add a node filter
/// filter.add_filter(OSMTagFilterType::Node, "foo".into(), Some("bar".into()));
/// // add a way filter
/// filter.add_filter(OSMTagFilterType::Way, "foo".into(), Some("bar".into()));
/// // add a relation filter
/// filter.add_filter(OSMTagFilterType::Relation, "foo".into(), Some("bar".into()));
/// // add a filter that effects all types
/// filter.add_filter(OSMTagFilterType::All, "foo".into(), Some("bar".into()));
/// ```
#[derive(Debug, Clone, Default)]
pub struct OSMTagFilter {
    all_filters: FilterMap,
    node_filters: FilterMap,
    way_filters: FilterMap,
    relation_filters: FilterMap,
}
impl OSMTagFilter {
    /// Add a filter
    pub fn add_filter(
        &mut self,
        filter_type: OSMTagFilterType,
        key: String,
        value: Option<String>,
    ) {
        let filter = self.get_filter(filter_type);
        filter.insert(key, value);
    }

    /// Check if a filter has been found
    /// @param filter_type - The filter type
    /// @param key - The key
    /// @param value - The value (optional)
    /// @returns - True if the filter has been found
    pub fn match_found(&mut self, filter_type: OSMTagFilterType, key: &str, value: &str) -> bool {
        // check all filters first
        if self.check_filter(OSMTagFilterType::All, key, value) {
            return true;
        }
        // check type-specific filters
        if filter_type != OSMTagFilterType::All && self.check_filter(filter_type, key, value) {
            return true;
        }

        false
    }

    /// Internal method to get the correct filter map
    fn get_filter(&mut self, filter_type: OSMTagFilterType) -> &mut FilterMap {
        match filter_type {
            OSMTagFilterType::All => &mut self.all_filters,
            OSMTagFilterType::Node => &mut self.node_filters,
            OSMTagFilterType::Way => &mut self.way_filters,
            OSMTagFilterType::Relation => &mut self.relation_filters,
        }
    }

    /// Internal method to check if a filter contains a match with a value
    fn check_filter(&mut self, filter_type: OSMTagFilterType, key: &str, value: &str) -> bool {
        let filter = self.get_filter(filter_type);
        match (filter.get(key), value) {
            (Some(Some(filter_value)), v) => filter_value == v,
            (Some(None), _) => true,
            _ => false,
        }
    }
}
