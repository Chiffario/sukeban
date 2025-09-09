use sukeban_types::{player::Player, ruleset::Ruleset};

/// Handler for GET /web/osu-osz2-getscores.php
async fn scores(
    player: Player,
    is_song_select: bool,
    leaderboard_type: u8,
    map_checksum: String,
    map_file_name: String,
    mode: Ruleset,
    mapset_id: u32,
    mods: i32,
    map_package_hash: String,
    aqn_files_found: bool,
) {
    todo!();
}
