fn match_opt_none() -> i8 {
    let x = None;
    x.unwrap_or(42)
}
