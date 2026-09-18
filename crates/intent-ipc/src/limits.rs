#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireLimits {
    pub max_control_frame_bytes: usize,
    pub max_artifact_frame_bytes: usize,
    pub max_json_depth: usize,
    pub max_collection_entries: usize,
    pub max_json_nodes: usize,
    pub max_frames_per_feed: usize,
}

impl Default for WireLimits {
    fn default() -> Self {
        Self {
            max_control_frame_bytes: 1024 * 1024,
            max_artifact_frame_bytes: 16 * 1024 * 1024,
            max_json_depth: 64,
            max_collection_entries: 16_384,
            max_json_nodes: 65_536,
            max_frames_per_feed: 256,
        }
    }
}

impl WireLimits {
    #[must_use]
    pub const fn for_tests() -> Self {
        Self {
            max_control_frame_bytes: 1024,
            max_artifact_frame_bytes: 4096,
            max_json_depth: 8,
            max_collection_entries: 16,
            max_json_nodes: 64,
            max_frames_per_feed: 8,
        }
    }
}
