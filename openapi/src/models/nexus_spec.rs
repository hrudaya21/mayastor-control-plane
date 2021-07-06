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

/// NexusSpec : User specification of a nexus.

/// User specification of a nexus.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NexusSpec {
    /// List of children.
    #[serde(rename = "children")]
    pub children: Vec<String>,
    /// Managed by our control plane
    #[serde(rename = "managed")]
    pub managed: bool,
    /// Node where the nexus should live.
    #[serde(rename = "node")]
    pub node: String,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<crate::models::NexusSpecOperation>,
    /// Volume which owns this nexus, if any
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<uuid::Uuid>,
    #[serde(rename = "share")]
    pub share: crate::models::Protocol,
    /// Size of the nexus.
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "state")]
    pub state: crate::models::SpecState,
    /// Nexus Id
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl NexusSpec {
    /// NexusSpec using only the required fields
    pub fn new(
        children: Vec<String>,
        managed: bool,
        node: String,
        share: crate::models::Protocol,
        size: i64,
        state: crate::models::SpecState,
        uuid: uuid::Uuid,
    ) -> NexusSpec {
        NexusSpec {
            children,
            managed,
            node,
            operation: None,
            owner: None,
            share,
            size,
            state,
            uuid,
        }
    }
    /// NexusSpec using all fields
    pub fn new_all(
        children: Vec<String>,
        managed: bool,
        node: String,
        operation: Option<crate::models::NexusSpecOperation>,
        owner: Option<uuid::Uuid>,
        share: crate::models::Protocol,
        size: i64,
        state: crate::models::SpecState,
        uuid: uuid::Uuid,
    ) -> NexusSpec {
        NexusSpec {
            children,
            managed,
            node,
            operation,
            owner,
            share,
            size,
            state,
            uuid,
        }
    }
}
