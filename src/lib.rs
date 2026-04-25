//! Container traits and registry — re-export shim.
//!
//! Historically this crate hosted the `Demuxer` / `Muxer` traits and
//! the `ContainerRegistry`. Those types moved to `oxideav-core` so the
//! unified `RuntimeContext` can hold all four registries without
//! cycle-through-codec-crate. This crate is now a thin re-export.

pub use oxideav_core::{
    ContainerProbeFn as ProbeFn, ContainerRegistry, Demuxer, Muxer, OpenDemuxerFn, OpenMuxerFn,
    ProbeData, ProbeScore, ReadSeek, WriteSeek, MAX_PROBE_SCORE, PROBE_SCORE_EXTENSION,
};

/// Compatibility module path for callers that imported through
/// `oxideav_container::registry::*`. The relocated types live in
/// [`oxideav_core::registry::container`].
pub mod registry {
    pub use oxideav_core::registry::container::{
        ContainerProbeFn as ProbeFn, ContainerRegistry, Demuxer, Muxer, OpenDemuxerFn, OpenMuxerFn,
        ProbeData, ProbeScore, ReadSeek, WriteSeek, MAX_PROBE_SCORE, PROBE_SCORE_EXTENSION,
    };
}
