pub fn rs_ts(
    raw_rs: &str
) -> &str {
    return raw_rs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_ts_works() {
        assert_eq!(rs_ts("ok"), "ok");
    }
}
