use serde::{Deserialize, Serialize};

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Primitive types supported by Properties
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PrimitiveValue {
    /// String type utf8 encoded
    String(String),
    /// unsigned 64 bit integer
    U64(u64),
    /// signed 64 bit integer
    I64(i64),
    /// floating point number
    F32(f32),
    /// double precision floating point number
    F64(f64),
    /// boolean
    Bool(bool),
    /// null
    Null,
}

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValuePrimitiveType {
    /// Primitive type
    Primitive(PrimitiveValue),
    /// Nested shape that can only contain primitives
    NestedPrimitive(BTreeMap<String, PrimitiveValue>),
}

/// Supports primitive types `string`, `number`, `boolean`, `null`
/// May be an array of those types, or an object of those types
/// Object keys are always strings, values can be any basic type, an array, or a nested object.
/// Array values must all be the same type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValueType {
    /// A primitive value
    Primitive(PrimitiveValue),
    /// An array of values
    Array(Vec<ValuePrimitiveType>),
    /// A nested object
    Nested(Value),
}

/// Shape design
pub type Value = BTreeMap<String, ValueType>;
/// Shape of a features properties object
pub type Properties = Value;
/// Shape of a feature's M-Values object
pub type MValue = Value;

/// LineString Properties Shape
pub type LineStringMValues = Vec<MValue>;
/// MultiLineString MValues Shape
pub type MultiLineStringMValues = Vec<LineStringMValues>;
/// Polygon MValues Shape
pub type PolygonMValues = Vec<LineStringMValues>;
/// MultiPolygon MValues Shape
pub type MultiPolygonMValues = Vec<PolygonMValues>;

/// All possible M-Value shapes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MValues {
    /// Single M-Value
    MValue(MValue),
    /// LineString M-Value
    LineStringMValues(LineStringMValues),
    /// MultiLineString M-Value
    MultiLineStringMValues(MultiLineStringMValues),
    /// Polygon M-Value
    PolygonMValues(PolygonMValues),
    /// MultiPolygon M-Value
    MultiPolygonMValues(MultiPolygonMValues),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_value() {
        let prim_value = PrimitiveValue::String("test".into());
        assert_eq!(prim_value, PrimitiveValue::String("test".into()));
        let prim_value = PrimitiveValue::U64(1);
        assert_eq!(prim_value, PrimitiveValue::U64(1));
        let prim_value = PrimitiveValue::I64(1);
        assert_eq!(prim_value, PrimitiveValue::I64(1));
        let prim_value = PrimitiveValue::F32(1.0);
        assert_eq!(prim_value, PrimitiveValue::F32(1.0));
        let prim_value = PrimitiveValue::F64(1.0);
        assert_eq!(prim_value, PrimitiveValue::F64(1.0));
        let prim_value = PrimitiveValue::Bool(true);
        assert_eq!(prim_value, PrimitiveValue::Bool(true));
        let prim_value = PrimitiveValue::Null;
        assert_eq!(prim_value, PrimitiveValue::Null);
    }

    #[test]
    fn primitive_string_serialize() {
        let prim_value = PrimitiveValue::String("test".into());
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "\"test\"");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::String("test".into()));
    }

    #[test]
    fn primitive_u64_serialize() {
        let prim_value = PrimitiveValue::U64(1);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "1");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::U64(1));
    }

    #[test]
    fn primitive_i64_serialize() {
        let prim_value = PrimitiveValue::I64(-1);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "-1");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::I64(-1));
    }

    #[test]
    fn primitive_f32_serialize() {
        let prim_value = PrimitiveValue::F32(1.0);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "1.0");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::F32(1.0));
    }

    #[test]
    fn primitive_f64_serialize() {
        let prim_value = PrimitiveValue::F64(-135435345435345345.0);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "-1.3543534543534534e17");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::F32(-1.3543534e17));
    }

    #[test]
    fn primitive_bool_serialize() {
        let prim_value = PrimitiveValue::Bool(true);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "true");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::Bool(true));
    }

    #[test]
    fn primitive_null_serialize() {
        let prim_value = PrimitiveValue::Null;
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "null");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::Null);
    }

    #[test]
    fn value_serialize() {
        let value: Value = BTreeMap::from([
            ("type".into(), ValueType::Primitive(PrimitiveValue::String("Point".into()))),
            ("coordinates".into(), ValueType::Primitive(PrimitiveValue::F32(1.0))),
        ]);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "{\"coordinates\":1.0,\"type\":\"Point\"}");
        let deserialize = serde_json::from_str::<Value>(&serialized).unwrap();
        assert_eq!(deserialize, value);

        let value_str = r#"
        {
            "class": "ocean",
            "offset": 22,
            "info": {
                "name": "Pacific Ocean",
                "value": 22.2
            }
        }
        "#;

        let deserialize = serde_json::from_str::<Value>(value_str).unwrap();
        assert_eq!(
            deserialize,
            BTreeMap::from([
                ("class".into(), ValueType::Primitive(PrimitiveValue::String("ocean".into()))),
                ("offset".into(), ValueType::Primitive(PrimitiveValue::U64(22))),
                (
                    "info".into(),
                    ValueType::Nested(BTreeMap::from([
                        (
                            "name".into(),
                            ValueType::Primitive(PrimitiveValue::String("Pacific Ocean".into()))
                        ),
                        ("value".into(), ValueType::Primitive(PrimitiveValue::F32(22.2))),
                    ]))
                ),
            ])
        );
    }
}
