use sukeban_types::{player::Player, ranked_status::RankedStatus, ruleset::Ruleset};

/// Handler for GET /web/osu-search.php
///
/// Returns a beatmaps
async fn search(
    player: Player,
    ranked_status: RankedStatus,
    query: String,
    mode: Ruleset,
    page: u32,
) {
    todo!()
}

/// Handler for GET /web/osu-search.php
///
/// Returns a beatmapsets
async fn mapset_search(player: Player, mapset_id: u32, map_id: u32, checksum: String) {
    todo!()
}
