use std::collections::HashMap;

type Point = (isize, isize);
type Sensors = HashMap<Point, Point>;

fn manhattan(p1: Point, p2: Point) -> isize {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as isize
}

fn get_sensors(input: String) -> Sensors {
    let regex = regex::Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

    let mut sensors: Sensors = Default::default();

    for caps in regex.captures_iter(&input) {
        let sensor = ((&caps["sensor_x"]).parse::<isize>().unwrap(), (&caps["sensor_y"]).parse::<isize>().unwrap());
        let beacon = ((&caps["beacon_x"]).parse::<isize>().unwrap(), (&caps["beacon_y"]).parse::<isize>().unwrap());

        sensors.insert(sensor, beacon);
    }

    sensors
}

pub(crate) fn part1(input: String) {
    let sensors = get_sensors(input);

    let min_x = sensors.iter().map(|(s, b)| s.0 - manhattan(*s, *b)).min().unwrap();
    let max_x = sensors.iter().map(|(s, b)| s.0 + manhattan(*s, *b)).max().unwrap();
    let mut non_beacons: usize = 0;
    let beacons: Vec<&Point> = sensors.values().collect();
    for x in min_x..=max_x {
        if !beacons.contains(&&(x, 2000000)) {
            for (sensor, beacon) in sensors.iter() {
                if manhattan(*sensor, *beacon) >= manhattan(*sensor, (x, 2000000)) {
                    non_beacons += 1;
                    break;
                }
            }
        }
    }

    println!("{non_beacons}");
}

pub(crate) fn part2(input: String) {
    let sensors = get_sensors(input);
    let mut x = 0isize;
    let mut y = 0isize;
    println!("{}", loop {
        let mut found = true;
        for (sensor, beacon) in sensors.iter() {
            let r = manhattan(*sensor, *beacon);
            if manhattan(*sensor, (x, y)) <= r {
                let yd = y.abs_diff(sensor.1) as isize;

                x = sensor.0 + r - yd;
                found = false;
                break;
            }
        }

        if found {
            break x * 4000000 + y;
        }

        x += 1;
        if x > 4000000 {
            x = 0;
            y += 1;
        }
    });
}