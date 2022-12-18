use std::ops::RangeInclusive;

type XYPoint = (i64, i64);

fn distance(a: &XYPoint, b: &XYPoint) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<(XYPoint, XYPoint, i64)>) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_distance = 0_i64;
    //"Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    let report: Vec<(XYPoint, XYPoint, i64)> = input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(':').unwrap();
            let (sensor_x, sensor_y) = sensor.split_once(',').unwrap();
            let (beacon_x, beacon_y) = beacon.split_once(',').unwrap();
            let sensor = (
                sensor_x[12..].parse::<i64>().unwrap(),
                sensor_y[3..].parse::<i64>().unwrap(),
            );
            let beacon = (
                beacon_x[24..].parse::<i64>().unwrap(),
                beacon_y[3..].parse::<i64>().unwrap(),
            );
            if sensor.0 < min_x {
                min_x = sensor.0
            }
            if sensor.0 > max_x {
                max_x = sensor.0
            }
            if beacon.0 < min_x {
                min_x = beacon.0
            }
            if beacon.0 > max_x {
                max_x = beacon.0
            }
            let distance = distance(&sensor, &beacon);
            if distance > max_distance {
                max_distance = distance;
            }
            (sensor, beacon, distance)
        })
        .collect::<Vec<_>>();
    (min_x, max_x, max_distance, report)
}

pub fn calc(input: &str, y: i64) -> Option<usize> {
    let (min_x, max_x, max_distance, report) = parse_input(input);
    ((min_x - max_distance)..=(max_x + max_distance))
        .map(|x| (x, y))
        .filter(|position| {
            report
                .iter()
                .all(|(sensor, beacon, _)| position != sensor && position != beacon)
        })
        .filter(|position| {
            report
                .iter()
                .any(|(sensor, _, reach)| distance(sensor, position) <= *reach)
        })
        .count()
        .into()
}


pub fn calc2(input: &str, search_range: RangeInclusive<i64>) -> Option<i64> {
    let (_, _, _, report) = parse_input(input);
    report
        .iter()
        // Remove sensors completely contained by others, just to reduce search space
        .filter(|(sensor_a, _, distance_a)| {
            !report.iter()
                .filter(|(sensor_b, _, _)| sensor_a != sensor_b)
                .any(|(sensor_b, _, distance_b)| {
                    distance(sensor_a, sensor_b) + distance_a <= *distance_b
                })
        })
        // Search only on the outside border of the sensor reach
        .flat_map(|(sensor, _, distance)| {
            (0..=*distance).flat_map(|i| {
                vec![
                    (sensor.0 + i + 1, sensor.1 + distance - i),
                    (sensor.0 - i - 1, sensor.1 - distance + i),
                    (sensor.0 + distance - i, sensor.1 + i + 1),
                    (sensor.0 - distance + i, sensor.1 - i - 1)
                ]
            }).collect::<Vec<_>>()
        })
        .filter(|(x, y)| search_range.contains(x) && search_range.contains(y))
        .filter(|position| {
            report
                .iter()
                .all(|(sensor, beacon, _)| position != sensor && position != beacon)
        })
        .filter(|position| {
            report
                .iter()
                .all(|(sensor, _, reach)| distance(sensor, position) > *reach)
        })
        .map(|(x, y)| {
            x * 4000000 + y
        })
        .next()
        .unwrap()
        .into()
}

pub fn part1(input: &str) -> Option<usize> {
    calc(input, 2000000)
}

pub fn part2(input: &str) -> Option<i64> {
    calc2(input, 0..=4000000)
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 15;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::calc(&utils::read_example(DAY, 1), 10), Some(26));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(5511201));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(
            super::calc2(&utils::read_example(DAY, 1), 0..=20),
            Some(56000011)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(11318723411840));
    }
}
