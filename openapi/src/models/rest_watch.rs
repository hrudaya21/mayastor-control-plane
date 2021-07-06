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

/// RestWatch : Watch Resource in the store

/// Watch Resource in the store
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RestWatch {
    /// callback used to notify the watcher of a change
    #[serde(rename = "callback")]
    pub callback: String,
    /// id of the resource to watch on
    #[serde(rename = "resource")]
    pub resource: String,
}

impl RestWatch {
    /// RestWatch using only the required fields
    pub fn new(callback: String, resource: String) -> RestWatch {
        RestWatch { callback, resource }
    }
    /// RestWatch using all fields
    pub fn new_all(callback: String, resource: String) -> RestWatch {
        RestWatch { callback, resource }
    }
}
