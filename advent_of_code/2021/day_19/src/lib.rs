use hashbrown::HashSet;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
pub struct Data {
    scans: Vec<Scan>,
}

#[derive(Debug, Clone, Default)]
struct Scan {
    grid: HashSet<Point>,
}

impl Scan {
    fn rotations(&self) -> impl Iterator<Item = Self> + '_ {
        // create an iterator of rotated scans
        (0..24).map(|idx| {
            // create a scan where every point is rotated the same way
            Scan {
                grid: self.grid.iter().map(|point| point.rotate(idx)).collect(),
            }
        })
    }

    fn try_merge(&mut self, other: &Scan) -> Option<Point> {
        // try to merge the other scan into "self" by trying all possible rotations of "other"
        for rotation in other.rotations() {
            let distances = self
                .grid
                .iter()
                .flat_map(|p1| rotation.grid.iter().map(move |p2| (p1, p2)))
                .map(|(p1, p2)| Point::new(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z));

            for point in distances {
                let dx = point.x;
                let dy = point.y;
                let dz = point.z;

                let translated = rotation
                    .grid
                    .iter()
                    .map(|point| Point::new(point.x + dx, point.y + dy, point.z + dz));

                let overlapping = translated
                    .clone()
                    .filter(|point| self.grid.contains(point))
                    .count();

                if overlapping >= 12 {
                    // this translation has significant overlap with self
                    // merge it into self, and return the scanner point
                    self.grid.extend(translated);
                    // the scanner point is the current distance point
                    return Some(point);
                }
            }
        }

        // all possible rotations of other could not be merged with self
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl Point {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    fn rotate(&self, idx: u8) -> Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        // all 24 possible rotations
        match idx {
            0 => Point::new(x, y, z),
            1 => Point::new(x, z, -y),
            2 => Point::new(x, -y, -z),
            3 => Point::new(x, -z, y),
            4 => Point::new(y, x, -z),
            5 => Point::new(y, z, x),
            6 => Point::new(y, -x, z),
            7 => Point::new(y, -z, -x),
            8 => Point::new(z, x, y),
            9 => Point::new(z, y, -x),
            10 => Point::new(z, -x, -y),
            11 => Point::new(z, -y, x),
            12 => Point::new(-x, y, -z),
            13 => Point::new(-x, z, y),
            14 => Point::new(-x, -y, z),
            15 => Point::new(-x, -z, -y),
            16 => Point::new(-y, x, z),
            17 => Point::new(-y, z, -x),
            18 => Point::new(-y, -x, -z),
            19 => Point::new(-y, -z, x),
            20 => Point::new(-z, x, -y),
            21 => Point::new(-z, y, x),
            22 => Point::new(-z, -x, y),
            23 => Point::new(-z, -y, -x),
            _ => unreachable!("Tried to create an invalid rotation"),
        }
    }

    fn manhattan(&self, other: &Self) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Data {
    pub fn part_one(&self) -> usize {
        let mut todo = self.scans.clone();
        // start the full scan off as the first scan
        let mut full_scan: Scan = todo.remove(0);

        while !todo.is_empty() {
            for i in (0..todo.len()).rev() {
                if let Some(_) = full_scan.try_merge(&todo[i]) {
                    todo.swap_remove(i);
                }
            }
        }

        full_scan.grid.len()
    }

    pub fn part_two(&self) -> i16 {
        let mut todo = self.scans.clone();
        // start the full scan off as the first scan
        let mut full_scan: Scan = todo.remove(0);
        let mut distances = Vec::new();
        // add the distance to the starting scanner
        distances.push(Point::new(0, 0, 0));

        while !todo.is_empty() {
            for idx in (0..todo.len()).rev() {
                if let Some(distance) = full_scan.try_merge(&todo[idx]) {
                    distances.push(distance);
                    todo.swap_remove(idx);
                }
            }
        }

        distances
            .iter()
            .enumerate()
            .flat_map(|(i, d1)| {
                distances.iter().enumerate().filter_map(move |(j, d2)| {
                    if i != j {
                        Some(d1.manhattan(d2))
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let scans: Vec<Scan> = input
            .trim()
            .split("\n\n")
            .map(|block| {
                let block: HashSet<Point> = block
                    .lines()
                    // skip the first line telling you what scanner this is
                    // that information is encoded in the index of the vector this block collects into
                    .skip(1)
                    .filter_map(|line| {
                        let (x, rest) = line.split_once(',')?;
                        let (y, z) = rest.split_once(',')?;
                        let x = x.parse().ok()?;
                        let y = y.parse().ok()?;
                        let z = z.parse().ok()?;
                        Some(Point { x, y, z })
                    })
                    .collect();
                Scan { grid: block }
            })
            .collect();

        Ok(Self { scans })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 79);
    }

    #[test]
    fn part_two_example() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 3621);
    }
}
