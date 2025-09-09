pub mod beatmap_request_form;
pub mod geolocation;
pub mod player;
pub mod privileges;
pub mod ranked_status;
pub mod ruleset;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
