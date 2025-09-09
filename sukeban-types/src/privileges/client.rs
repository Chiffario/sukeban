use bitflags::bitflags;

pub struct ClientPrivileges(u8);
bitflags! {
    impl ClientPrivileges: u8 {
        const Player = 1 << 0;
        const Moderator = 1 << 1;
        const Supporter = 1 << 2;
        const Owner = 1 << 3;
        const Developer = 1 << 4;
        // Not used by the client
        const Tournament = 1 << 5;
    }

}
