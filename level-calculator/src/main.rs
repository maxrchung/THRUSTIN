/// Goal: About 10,000 hours invested to reach level 100

/// Each level requires a certain amount of experience that increases per level based on an exponential factor
/// This test utility allows you to configure the exponential factor and the experience per hour and see rough results

/// I want to calculate experience in such a manner:
/// Experience is only calculated for players at the end of a game
/// Everyone gets participation experience, based on the number of people in the room 
///     e.g. if there are 5 people, there is a free 5 participation experience awarded for finishing a game
/// In addition, you get experience for any points you personally won
/// If you are the winner of the game, you get additional experience for points scored by other people
///     e.g. if there are 5 people in a room and you win the game with 7 points while the others got 1 points each,
///     you get a total of 5 (participation) + 7 (your points) + 4 (other points) = 16 experience

/// Quick calculations with some estimated (est) generalizations:
/// 7 points max
/// 5 players in lobby (est)
/// 7 points winner, 4 points (est) other players
/// Winner gets 5 (participation) + 7 (won points) + 16 (other points) = 23 experience
/// Loser gets 5 (participation) + 4 (won points) = 9 experience
/// (est) experience per game = ((9 * 4) + 28) / 5 = 12.8
/// (est) games per hour = 5
/// (est) experience per hour = 12.8 * 5 = 64

static EXPONENT: f32 = 2.15;
static EXP_PER_HR: u32 = 64;

fn calculate_exp(level: u32) -> u32 {
    let level = level as f32;
    level.powf(EXPONENT).round() as u32
}

fn main() {
    let mut total_hours = 0;
    for level in 1..101 {
        let exp = calculate_exp(level);
        let hours = exp / EXP_PER_HR;
        total_hours += hours;
        println!("Level: {}\tExperience: {}\tHours: {}\tTotal Hours: {}", level, exp, hours, total_hours);
    }
}

/*
Level: 1        Experience: 1   Hours: 0        Total Hours: 0
Level: 2        Experience: 4   Hours: 0        Total Hours: 0
Level: 3        Experience: 11  Hours: 0        Total Hours: 0
Level: 4        Experience: 20  Hours: 0        Total Hours: 0
Level: 5        Experience: 32  Hours: 0        Total Hours: 0
Level: 6        Experience: 47  Hours: 0        Total Hours: 0
Level: 7        Experience: 66  Hours: 1        Total Hours: 1
Level: 8        Experience: 87  Hours: 1        Total Hours: 2
Level: 9        Experience: 113 Hours: 1        Total Hours: 3
Level: 10       Experience: 141 Hours: 2        Total Hours: 5
Level: 11       Experience: 173 Hours: 2        Total Hours: 7
Level: 12       Experience: 209 Hours: 3        Total Hours: 10
Level: 13       Experience: 248 Hours: 3        Total Hours: 13
Level: 14       Experience: 291 Hours: 4        Total Hours: 17
Level: 15       Experience: 338 Hours: 5        Total Hours: 22
Level: 16       Experience: 388 Hours: 6        Total Hours: 28
Level: 17       Experience: 442 Hours: 6        Total Hours: 34
Level: 18       Experience: 500 Hours: 7        Total Hours: 41
Level: 19       Experience: 561 Hours: 8        Total Hours: 49
Level: 20       Experience: 627 Hours: 9        Total Hours: 58
Level: 21       Experience: 696 Hours: 10       Total Hours: 68
Level: 22       Experience: 770 Hours: 12       Total Hours: 80
Level: 23       Experience: 847 Hours: 13       Total Hours: 93
Level: 24       Experience: 928 Hours: 14       Total Hours: 107
Level: 25       Experience: 1013        Hours: 15       Total Hours: 122
Level: 26       Experience: 1102        Hours: 17       Total Hours: 139
Level: 27       Experience: 1195        Hours: 18       Total Hours: 157
Level: 28       Experience: 1292        Hours: 20       Total Hours: 177
Level: 29       Experience: 1394        Hours: 21       Total Hours: 198
Level: 30       Experience: 1499        Hours: 23       Total Hours: 221
Level: 31       Experience: 1609        Hours: 25       Total Hours: 246
Level: 32       Experience: 1722        Hours: 26       Total Hours: 272
Level: 33       Experience: 1840        Hours: 28       Total Hours: 300
Level: 34       Experience: 1962        Hours: 30       Total Hours: 330
Level: 35       Experience: 2088        Hours: 32       Total Hours: 362
Level: 36       Experience: 2218        Hours: 34       Total Hours: 396
Level: 37       Experience: 2353        Hours: 36       Total Hours: 432
Level: 38       Experience: 2492        Hours: 38       Total Hours: 470
Level: 39       Experience: 2635        Hours: 41       Total Hours: 511
Level: 40       Experience: 2782        Hours: 43       Total Hours: 554
Level: 41       Experience: 2934        Hours: 45       Total Hours: 599
Level: 42       Experience: 3090        Hours: 48       Total Hours: 647
Level: 43       Experience: 3251        Hours: 50       Total Hours: 697
Level: 44       Experience: 3415        Hours: 53       Total Hours: 750
Level: 45       Experience: 3584        Hours: 56       Total Hours: 806
Level: 46       Experience: 3758        Hours: 58       Total Hours: 864
Level: 47       Experience: 3936        Hours: 61       Total Hours: 925
Level: 48       Experience: 4118        Hours: 64       Total Hours: 989
Level: 49       Experience: 4304        Hours: 67       Total Hours: 1056
Level: 50       Experience: 4496        Hours: 70       Total Hours: 1126
Level: 51       Experience: 4691        Hours: 73       Total Hours: 1199
Level: 52       Experience: 4891        Hours: 76       Total Hours: 1275
Level: 53       Experience: 5096        Hours: 79       Total Hours: 1354
Level: 54       Experience: 5305        Hours: 82       Total Hours: 1436
Level: 55       Experience: 5518        Hours: 86       Total Hours: 1522
Level: 56       Experience: 5736        Hours: 89       Total Hours: 1611
Level: 57       Experience: 5958        Hours: 93       Total Hours: 1704
Level: 58       Experience: 6185        Hours: 96       Total Hours: 1800
Level: 59       Experience: 6417        Hours: 100      Total Hours: 1900
Level: 60       Experience: 6653        Hours: 103      Total Hours: 2003
Level: 61       Experience: 6894        Hours: 107      Total Hours: 2110
Level: 62       Experience: 7139        Hours: 111      Total Hours: 2221
Level: 63       Experience: 7389        Hours: 115      Total Hours: 2336
Level: 64       Experience: 7643        Hours: 119      Total Hours: 2455
Level: 65       Experience: 7902        Hours: 123      Total Hours: 2578
Level: 66       Experience: 8166        Hours: 127      Total Hours: 2705
Level: 67       Experience: 8435        Hours: 131      Total Hours: 2836
Level: 68       Experience: 8708        Hours: 136      Total Hours: 2972
Level: 69       Experience: 8985        Hours: 140      Total Hours: 3112
Level: 70       Experience: 9267        Hours: 144      Total Hours: 3256
Level: 71       Experience: 9554        Hours: 149      Total Hours: 3405
Level: 72       Experience: 9846        Hours: 153      Total Hours: 3558
Level: 73       Experience: 10142       Hours: 158      Total Hours: 3716
Level: 74       Experience: 10444       Hours: 163      Total Hours: 3879
Level: 75       Experience: 10749       Hours: 167      Total Hours: 4046
Level: 76       Experience: 11060       Hours: 172      Total Hours: 4218
Level: 77       Experience: 11375       Hours: 177      Total Hours: 4395
Level: 78       Experience: 11695       Hours: 182      Total Hours: 4577
Level: 79       Experience: 12020       Hours: 187      Total Hours: 4764
Level: 80       Experience: 12349       Hours: 192      Total Hours: 4956
Level: 81       Experience: 12684       Hours: 198      Total Hours: 5154
Level: 82       Experience: 13023       Hours: 203      Total Hours: 5357
Level: 83       Experience: 13367       Hours: 208      Total Hours: 5565
Level: 84       Experience: 13715       Hours: 214      Total Hours: 5779
Level: 85       Experience: 14069       Hours: 219      Total Hours: 5998
Level: 86       Experience: 14427       Hours: 225      Total Hours: 6223
Level: 87       Experience: 14790       Hours: 231      Total Hours: 6454
Level: 88       Experience: 15158       Hours: 236      Total Hours: 6690
Level: 89       Experience: 15531       Hours: 242      Total Hours: 6932
Level: 90       Experience: 15908       Hours: 248      Total Hours: 7180
Level: 91       Experience: 16291       Hours: 254      Total Hours: 7434
Level: 92       Experience: 16678       Hours: 260      Total Hours: 7694
Level: 93       Experience: 17070       Hours: 266      Total Hours: 7960
Level: 94       Experience: 17467       Hours: 272      Total Hours: 8232
Level: 95       Experience: 17869       Hours: 279      Total Hours: 8511
Level: 96       Experience: 18276       Hours: 285      Total Hours: 8796
Level: 97       Experience: 18688       Hours: 292      Total Hours: 9088
Level: 98       Experience: 19105       Hours: 298      Total Hours: 9386
Level: 99       Experience: 19526       Hours: 305      Total Hours: 9691
Level: 100      Experience: 19953       Hours: 311      Total Hours: 10002
*/