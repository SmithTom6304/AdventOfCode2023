pub fn distance(hold_time: u32, total_time: u32) -> u32 {
    let move_time = total_time - hold_time;
    let speed = hold_time;
    speed * move_time
}

pub fn iterate_distances(total_time: u32) -> impl Iterator<Item = u32> {
    (0..total_time).map(move |hold_time| distance(hold_time, total_time))
}

pub fn distances_greater_than_record(total_time: u32, record_distance: u32) -> impl Iterator {
    iterate_distances(total_time).filter(move |distance| *distance > record_distance)
}
