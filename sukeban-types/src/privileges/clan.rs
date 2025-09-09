use bitflags::bitflags;
pub struct ClanPrivileges(u8);

bitflags! {
    impl ClanPrivileges: u8 {
        const Member = 1;
        const Owner = 2;
    }
}
