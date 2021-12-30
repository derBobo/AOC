use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

struct SnailfishSum {
    left: Option<Box<SnailfishSum>>,
    right: Option<Box<SnailfishSum>>,
    value: u16,
}

impl std::fmt::Display for SnailfishSum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.left.is_none() || self.right.is_none() {
            write!(f, "{}", self.value)
        } else {
            write!(
                f,
                "[{},{}]",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            )
        }
    }
}
impl Clone for SnailfishSum {
    fn clone(&self) -> Self {
        if self.left.is_none() || self.right.is_none() {
            SnailfishSum {
                left: None,
                right: None,
                value: self.value,
            }
        } else {
            SnailfishSum {
                left: Some(Box::new(self.left.as_ref().unwrap().deref().clone())),
                right: Some(Box::new(self.right.as_ref().unwrap().deref().clone())),
                value: self.value,
            }
        }
    }
}
impl SnailfishSum {
    fn reduce(&self) -> SnailfishSum {
        let mut new_sum = self.clone();
        //println!("Before reduction: {}", new_sum);
        loop {
            let res_exp = new_sum.explode(0);
            new_sum = res_exp.1;
            if res_exp.2 {
                //println!("After explosion: {}", new_sum);
                continue;
            }
            let res_split = new_sum.split();
            new_sum = res_split.1;
            if res_split.0 {
                //println!("After splitting: {}", new_sum);
                continue;
            }
            break;
        }
        new_sum
    }
    fn split(&self) -> (bool, SnailfishSum) {
        return if self.right.is_none() || self.left.is_none() {
            if self.value < 10 {
                (false, self.clone())
            } else if self.value % 2 == 0 {
                (
                    true,
                    SnailfishSum {
                        left: Some(Box::new(SnailfishSum {
                            left: None,
                            right: None,
                            value: self.value / 2,
                        })),
                        right: Some(Box::new(SnailfishSum {
                            left: None,
                            right: None,
                            value: self.value / 2,
                        })),
                        value: 0,
                    },
                )
            } else {
                (
                    true,
                    SnailfishSum {
                        left: Some(Box::new(SnailfishSum {
                            left: None,
                            right: None,
                            value: self.value / 2,
                        })),
                        right: Some(Box::new(SnailfishSum {
                            left: None,
                            right: None,
                            value: self.value / 2 + 1,
                        })),
                        value: 0,
                    },
                )
            }
        } else {
            let mut res = self.left.as_ref().unwrap().split();
            if res.0 {
                return (
                    true,
                    SnailfishSum {
                        left: Some(Box::new(res.1)),
                        right: Some(Box::new(self.right.as_ref().unwrap().deref().clone())),
                        value: self.value,
                    },
                );
            }
            res = self.right.as_ref().unwrap().split();
            if res.0 {
                return (
                    true,
                    SnailfishSum {
                        left: Some(Box::new(self.left.as_ref().unwrap().deref().clone())),
                        right: Some(Box::new(res.1)),
                        value: self.value,
                    },
                );
            }

            (false, self.clone())
        };
    }
    fn magnitude(&self) -> u32 {
        if self.left.is_none() || self.right.is_none() {
            self.value as u32
        } else {
            3 * self.left.as_ref().unwrap().magnitude()
                + 2 * self.right.as_ref().unwrap().magnitude()
        }
    }
    fn explode(&mut self, depth: u32) -> (Option<u16>, SnailfishSum, bool, Option<u16>) {
        if self.left.is_none() || self.right.is_none() {
            (None, self.clone(), false, None)
        } else if depth > 3
            && (self.left.as_ref().unwrap().left.is_none()
                || self.left.as_ref().unwrap().right.is_none())
            && (self.right.as_ref().unwrap().left.is_none()
                || self.right.as_ref().unwrap().right.is_none())
        {
            (
                Some(self.left.as_ref().unwrap().value),
                SnailfishSum {
                    left: None,
                    right: None,
                    value: 0,
                },
                true,
                Some(self.right.as_ref().unwrap().value),
            )
        } else {
            let mut res = self.left.as_ref().unwrap().clone().explode(depth + 1);
            if res.2 {
                //if we come from the left we cannot add the left value
                return (
                    res.0,
                    SnailfishSum {
                        left: Some(Box::new(res.1)),
                        right: Some(Box::new(
                            self.right
                                .as_ref()
                                .unwrap()
                                .add_explode_right(res.3.unwrap()),
                        )),
                        value: self.value,
                    },
                    true,
                    Some(0),
                );
            }
            res = self.right.as_ref().unwrap().clone().explode(depth + 1);
            if res.2 {
                // if we come from the right,we cannot add the right value
                return (
                    Some(0),
                    SnailfishSum {
                        left: Some(Box::new(
                            self.left.as_ref().unwrap().add_explode_left(res.0.unwrap()),
                        )),
                        right: Some(Box::new(res.1)),
                        value: self.value,
                    },
                    true,
                    res.3,
                );
            }

            return (None, self.clone(), false, None);
        }
    }
    fn add_explode_right(&self, value_to_add: u16) -> SnailfishSum {
        if self.left.is_none() {
            SnailfishSum {
                left: None,
                right: None,
                value: self.value + value_to_add,
            }
        } else {
            SnailfishSum {
                left: Some(Box::new(
                    self.left.as_ref().unwrap().add_explode_right(value_to_add),
                )),
                right: Some(Box::new(self.right.as_ref().unwrap().deref().clone())),
                value: self.value,
            }
        }
    }
    fn add_explode_left(&self, value_to_add: u16) -> SnailfishSum {
        if self.right.is_none() {
            SnailfishSum {
                left: None,
                right: None,
                value: self.value + value_to_add,
            }
        } else {
            SnailfishSum {
                left: Some(Box::new(self.left.as_ref().unwrap().deref().clone())),
                right: Some(Box::new(
                    self.right.as_ref().unwrap().add_explode_left(value_to_add),
                )),
                value: self.value,
            }
        }
    }
}

fn main() {
    let mut sum = SnailfishSum {
        left: None,
        right: None,
        value: 0,
    };
    let path = "INPUT18.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let input: Vec<SnailfishSum> = buffered
        .lines()
        .map(|x| parse(x.unwrap().to_string()))
        .collect();
    for i in 0..input.len() {
        if sum.left.is_none() && sum.right.is_none() && sum.value == 0 {
            sum = input[i].clone();
        } else {
            sum = SnailfishSum {
                left: Some(Box::new(sum)),
                right: Some(Box::new(input[i].clone())),
                value: 0,
            };
            sum = sum.reduce()
        }
    }
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            };
            let mut cur = SnailfishSum {
                left: Some(Box::new(input[i].clone())),
                right: Some(Box::new(input[j].clone())),
                value: 0,
            };
            cur = cur.reduce();
            max = std::cmp::max(max, cur.magnitude());
        }
    }
    println!("{}", sum.magnitude());
    print!("{}", max);
}
fn parse(input: String) -> SnailfishSum {
    if input.starts_with("[") {
        let mut number_of_brackets = 0;
        let mut index = 0;
        for c in input.chars() {
            if c == '[' {
                number_of_brackets += 1;
            } else if c == ']' {
                number_of_brackets -= 1;
            } else if number_of_brackets == 1 && c == ',' {
                break;
            }
            index += 1;
        }
        SnailfishSum {
            left: Some(Box::new(parse(input[1..index].to_string()))),
            right: Some(Box::new(parse(
                input[index + 1..input.len() - 1].to_string(),
            ))),
            value: 0,
        }
    } else {
        SnailfishSum {
            left: None,
            right: None,
            value: input.parse().unwrap(),
        }
    }
}
