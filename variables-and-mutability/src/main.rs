const TOUCHDOWN_POINTS: i8 = 6;

fn main() {
    let season: &str = "autumn";

    #[allow(unused_assignments)]
    let mut points_scored: i32 = 28;

    points_scored = 35;

    #[allow(unused_variables)]
    let event_time = "06:00";
    let event_time = 6;

    println!(
        "In the {season} season, the team scored {} points with {2} touchdown points. The game started at {1} o'clock.",
        points_scored, event_time, TOUCHDOWN_POINTS
    );

    let _favourite_beverage = "milk";
}
