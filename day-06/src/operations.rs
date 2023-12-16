pub fn distance(hold_time: u64, total_time: u64) -> u64 {
    let move_time = total_time - hold_time;
    let speed = hold_time;
    speed * move_time
}

pub fn iterate_distances(total_time: u64) -> impl Iterator<Item = u64> {
    (0..total_time).map(move |hold_time| distance(hold_time, total_time))
}

pub fn distances_greater_than_record(total_time: u64, record_distance: u64) -> impl Iterator {
    iterate_distances(total_time).filter(move |distance| *distance > record_distance)
}
