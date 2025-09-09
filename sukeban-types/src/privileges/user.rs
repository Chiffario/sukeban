use bitflags::bitflags;

pub struct UserPrivileges(u16);

bitflags! {
    impl UserPrivileges: u16 {
        const Unrestricted = 1 << 0;
        const Verified = 1 << 1;
        const Whitelisted = 1 << 2;
        const Supporter = 1 << 4;
        const Premium = 1 << 5;
        const Alumni = 1 << 7;
        const TourneyManager = 1 << 10;
        const BeatmapNominator = 1 << 11;
        const Moderator = 1 << 12;
        const Administrator = 1 << 13;
        const Developer = 1 << 14;

        const Donator = 1 << 4 | 1 << 5;
        const Staff = 1 << 12 | 1 << 13| 1 << 14;
    }
}
