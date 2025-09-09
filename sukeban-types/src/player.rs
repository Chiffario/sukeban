use std::str::Bytes;

use crate::privileges::{clan::ClanPrivileges, user::UserPrivileges};

struct Player {
    id: u32,
    name: String,
    privileges: UserPrivileges,
    pw_bcrypt: Option<Vec<u8>>,
    token: String,
    clan_id: Option<u32>,
    clan_priveleges: Option<ClanPrivileges>,
    geolocation: Geolocation,
    utc_offset: i8,
    private_pms: bool,
    silence_end: u64,   // TODO: To timestamp
    supporter_end: u64, // TODO: To timestamp
    client_details: Option<ClientDetails>,
    login_time: f64, // TODO: To timestamp
    is_bot_client: bool,
    is_tourney_client: bool,
    api_key: Option<String>,
}
