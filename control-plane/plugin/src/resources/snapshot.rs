use crate::{
    operations::{GetSnapshots, PluginResult},
    resources::{error::Error, utils, utils::optional_cell, SnapshotId, VolumeId},
    rest_wrapper::RestClient,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::resources::utils::{CreateRow, GetHeaderRow};
use prettytable::Row;

/// A collection of VolumeSnapshot resource.
#[derive(clap::Args, Debug)]
pub struct VolumeSnapshots {}

/// Volume Snapshot args.
#[derive(Debug, Clone, clap::Args)]
pub struct VolumeSnapshotArgs {
    /// Uuid of the volume (Optional).
    #[clap(long)]
    volume: Option<VolumeId>,
    /// Uuid of the snapshot (Optional).
    #[clap(long)]
    snapshot: Option<SnapshotId>,
}

impl VolumeSnapshotArgs {
    /// Get the volume id from args.
    pub fn volume(&self) -> Option<VolumeId> {
        self.volume
    }

    /// Get the snapshot id from args.
    pub fn snapshot(&self) -> Option<SnapshotId> {
        self.snapshot
    }
}

impl CreateRow for openapi::models::VolumeSnapshot {
    fn row(&self) -> Row {
        let meta = &self.definition.metadata;
        let state = &self.state;
        let timestamp =
            state
                .timestamp
                .as_ref()
                .map(|timestamp| match DateTime::<Utc>::from_str(timestamp) {
                    Ok(timestamp) => timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    Err(_) => timestamp.to_string(),
                });

        row![
            state.uuid,
            optional_cell(timestamp),
            ::utils::bytes::into_human(meta.spec_size),
            ::utils::bytes::into_human(state.allocated_size),
            ::utils::bytes::into_human(meta.total_allocated_size),
            state.source_volume,
            self.definition.metadata.num_restores,
            self.definition.metadata.num_snapshot_replicas
        ]
    }
}

#[async_trait(?Send)]
impl GetSnapshots for VolumeSnapshots {
    type SourceID = Option<VolumeId>;
    type ResourceID = Option<SnapshotId>;
    async fn get_snapshots(
        volid: &Self::SourceID,
        snapid: &Self::ResourceID,
        output: &utils::OutputFormat,
    ) -> PluginResult {
        match get_snapshots(volid, snapid).await {
            Ok(snapshots) => {
                // Print table, json or yaml based on output format.
                utils::print_table(output, snapshots);
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

impl GetHeaderRow for openapi::models::VolumeSnapshot {
    fn get_header_row(&self) -> Row {
        (*utils::SNAPSHOT_HEADERS).clone()
    }
}

async fn get_snapshots(
    volid: &Option<VolumeId>,
    snapid: &Option<SnapshotId>,
) -> Result<Vec<openapi::models::VolumeSnapshot>, Error> {
    let max_entries = 100;
    let mut starting_token = Some(0);
    let mut snapshots = Vec::with_capacity(max_entries as usize);

    // The last paginated request will set the `starting_token` to `None`.
    while starting_token.is_some() {
        match RestClient::client()
            .snapshots_api()
            .get_volumes_snapshots(max_entries, snapid.as_ref(), volid.as_ref(), starting_token)
            .await
        {
            Ok(snaps) => {
                let s = snaps.into_body();
                snapshots.extend(s.entries);
                starting_token = s.next_token;
            }
            Err(e) => {
                return Err(Error::ListSnapshotsError { source: e });
            }
        }
    }

    Ok(snapshots)
}
