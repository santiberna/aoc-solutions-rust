fn merge_range(a: &(i64, i64), b: &(i64, i64)) -> Option<(i64, i64)> {
    let (a_start, a_end) = *a;
    let (b_start, b_end) = *b;
    if a_end < b_start || b_end < a_start {
        return None;
    }
    Some((a_start.min(b_start), a_end.max(b_end)))
}
