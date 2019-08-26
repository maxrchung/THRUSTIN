/// Goal: About 10,000 hours invested to reach level 100

/// Each level requires a certain amount of experience that increases per level based on an exponential factor
/// This test utility allows you to configure the exponential factor and the points per hour and see rough results
/// Experience is only calculated for players at the end of a game

/// I want to calculate experience in such a manner:
/// Everyone gets participation experience, based on the number of people in the room, 
///     e.g. if there are 5 people, there is a free 5 participation experience awarded for finishing a game
/// In addition, you get experience for any points you personally won
/// If you are the winner of the game, you get additional experience for any points scored by other people
///     e.g. if there are 5 people in a room and you win the game with 7 points while the others got 1 points each,
///     you get a total of 5 (participation) + 7 (your points) + 4 (other points) = 16 experience

/// Quick calculations with some estimated generalizations:
/// 7 points max
/// 5 players in room (est)
/// 7 points winner, 4 points (est) other players
/// Winner gets 5 (participation) + 7 (won points) + 16 (other points) = 23 experience
/// Loser gets 5 (participation) + 4 (won points) = 9 experience
/// (est) experience per game = ((9 * 4) + 28) / 5 = 12.8
/// (est) games per hour = 5
/// (est) points per hour = 12.8 * 5 = 64

static EXPONENT: f32 = 2.15;
static PTS_PER_HR: u32 = 64;

fn calculate_exp(level: u32) -> u32 {
    let level = level as f32;
    level.powf(EXPONENT).round() as u32
}

fn main() {
    let mut total_hours = 0;
    for level in 1..101 {
        let exp = calculate_exp(level);
        let hours = exp / PTS_PER_HR;
        total_hours += hours;
        println!("Level: {}\tExperience: {}\tHours: {}\tTotal Hours: {}", level, exp, hours, total_hours);
    }
}
