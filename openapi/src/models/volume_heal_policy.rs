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

/// VolumeHealPolicy : Volume Healing policy used to determine if and how to replace a replica

/// Volume Healing policy used to determine if and how to replace a replica
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VolumeHealPolicy {
    /// the server will attempt to heal the volume by itself  the client should not attempt to do the same if this is enabled
    #[serde(rename = "self_heal")]
    pub self_heal: bool,
    /// topology to choose a replacement replica for self healing  (overrides the initial creation topology)
    #[serde(rename = "topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<crate::models::Topology>,
}

impl VolumeHealPolicy {
    /// VolumeHealPolicy using only the required fields
    pub fn new(self_heal: bool) -> VolumeHealPolicy {
        VolumeHealPolicy {
            self_heal,
            topology: None,
        }
    }
    /// VolumeHealPolicy using all fields
    pub fn new_all(self_heal: bool, topology: Option<crate::models::Topology>) -> VolumeHealPolicy {
        VolumeHealPolicy {
            self_heal,
            topology,
        }
    }
}
