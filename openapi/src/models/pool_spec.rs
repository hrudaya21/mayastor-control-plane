#![allow(
    clippy::too_many_arguments,
    clippy::new_without_default,
    non_camel_case_types
)]
/*
 * Mayastor RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

/// PoolSpec : User specification of a pool.

/// User specification of a pool.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PoolSpec {
    /// absolute disk paths claimed by the pool
    #[serde(rename = "disks")]
    pub disks: Vec<String>,
    /// id of the pool
    #[serde(rename = "id")]
    pub id: String,
    /// Pool labels.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// id of the mayastor instance
    #[serde(rename = "node")]
    pub node: String,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<crate::models::PoolSpecOperation>,
    #[serde(rename = "state")]
    pub state: crate::models::SpecState,
}

impl PoolSpec {
    /// PoolSpec using only the required fields
    pub fn new(
        disks: Vec<String>,
        id: String,
        labels: Vec<String>,
        node: String,
        state: crate::models::SpecState,
    ) -> PoolSpec {
        PoolSpec {
            disks,
            id,
            labels,
            node,
            operation: None,
            state,
        }
    }
    /// PoolSpec using all fields
    pub fn new_all(
        disks: Vec<String>,
        id: String,
        labels: Vec<String>,
        node: String,
        operation: Option<crate::models::PoolSpecOperation>,
        state: crate::models::SpecState,
    ) -> PoolSpec {
        PoolSpec {
            disks,
            id,
            labels,
            node,
            operation,
            state,
        }
    }
}
