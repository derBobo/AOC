use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::current;

fn main() {
    let path = "INPUT15.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let riskMatrix: Vec<Vec<u32>> = buffered
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let part1 = solveDijkstra(&riskMatrix);
    println!("{}", part1);

    let mut extendedMatrix: Vec<Vec<u32>> = Vec::new();
    for x in &riskMatrix {
        extendedMatrix.push(Vec::new());
        let length = extendedMatrix.len();
        for i in 0..5 {
            for y in x {
                let mut newValue = y + i;
                if newValue > 9 {
                    newValue -= 9;
                }
                extendedMatrix[length - 1].push(newValue);
            }
        }
    }
    let length = extendedMatrix.len();
    for i in 1..5 {
        for line in 0..length {
            extendedMatrix.push(
                extendedMatrix[line]
                    .iter()
                    .map(|x| if x + i > 9 { x + i - 9 } else { x + i })
                    .collect(),
            );
        }
    }

    let part2 = solveDijkstra(&extendedMatrix);
    println!("{}", part2);
}
fn solveDijkstra(riskMatrix: &Vec<Vec<u32>>) -> u32 {
    let mut solved = false;
    let mut checkedPoints: Vec<([usize; 2], u32)> = Vec::new();
    let mut pointsToCheck: Vec<([usize; 2], u32)> = Vec::new();
    pointsToCheck.push(([0, 0], 0));
    while !solved {
        pointsToCheck.sort_by(|a, b| b.1.cmp(&a.1));
        let currentPoint = pointsToCheck.pop().unwrap();
        if currentPoint.0[0] == riskMatrix.len() - 1 && currentPoint.0[1] == riskMatrix[0].len() - 1
        {
            solved = true;
        }
        if currentPoint.0[0] as i32 - 1 >= 0
            && !checkedPoints
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] - 1 && x.0[1] == currentPoint.0[1])
            && !pointsToCheck
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] - 1 && x.0[1] == currentPoint.0[1])
        {
            pointsToCheck.push((
                [currentPoint.0[0] - 1, currentPoint.0[1]],
                currentPoint.1 + riskMatrix[currentPoint.0[0] - 1][currentPoint.0[1]],
            ));
        }
        if currentPoint.0[0] + 1 < riskMatrix.len()
            && !checkedPoints
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] + 1 && x.0[1] == currentPoint.0[1])
            && !pointsToCheck
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] + 1 && x.0[1] == currentPoint.0[1])
        {
            pointsToCheck.push((
                [currentPoint.0[0] + 1, currentPoint.0[1]],
                currentPoint.1 + riskMatrix[currentPoint.0[0] + 1][currentPoint.0[1]],
            ));
        }
        if currentPoint.0[1] as i32 - 1 >= 0
            && !checkedPoints
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] && x.0[1] == currentPoint.0[1] - 1)
            && !pointsToCheck
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] && x.0[1] == currentPoint.0[1] - 1)
        {
            pointsToCheck.push((
                [currentPoint.0[0], currentPoint.0[1] - 1],
                currentPoint.1 + riskMatrix[currentPoint.0[0]][currentPoint.0[1] - 1],
            ));
        }
        if currentPoint.0[1] + 1 < riskMatrix[0].len()
            && !checkedPoints
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] && x.0[1] == currentPoint.0[1] + 1)
            && !pointsToCheck
                .iter()
                .any(|x| x.0[0] == currentPoint.0[0] && x.0[1] == currentPoint.0[1] + 1)
        {
            pointsToCheck.push((
                [currentPoint.0[0], currentPoint.0[1] + 1],
                currentPoint.1 + riskMatrix[currentPoint.0[0]][currentPoint.0[1] + 1],
            ));
        }
        checkedPoints.push(currentPoint);
    }
    let mut pathsToTarget: Vec<&([usize; 2], u32)> = checkedPoints
        .iter()
        .filter(|x| x.0[0] == riskMatrix.len() - 1 && x.0[1] == riskMatrix[0].len() - 1)
        .collect();
    pathsToTarget.sort_by(|a, b| a.1.cmp(&b.1));
    pathsToTarget.last().unwrap().1
}
