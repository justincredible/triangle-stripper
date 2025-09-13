use std::collections::VecDeque;
use std::fmt::{Debug, Error, Formatter};

use crate::triangle::Triangle;

pub struct Failure;

impl Debug for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "no strip found!")
    }
}

pub fn strip(triangle_list: VecDeque<Triangle>) -> Result<Vec<u32>, Failure> {
    let full_list = triangle_list.clone();

    for (i, triangle) in triangle_list.into_iter().enumerate() {
        let mut strip = vec!(triangle);
        let mut candidates = full_list.clone();
        candidates.swap_remove_back(i);
        if find(&mut strip, &mut candidates, true) {
            return Ok(collapse(strip));
        }

        strip = vec!(triangle.rotate());
        candidates = full_list.clone();
        candidates.swap_remove_back(i);
        if find(&mut strip, &mut candidates, true) {
            return Ok(collapse(strip));
        }

        strip = vec!(triangle.rotate().rotate());
        candidates = full_list.clone();
        candidates.swap_remove_back(i);
        if find(&mut strip, &mut candidates, true) {
            return Ok(collapse(strip));
        }
    }

    Err(Failure)
}

fn find(strip: &mut Vec<Triangle>, candidates: &mut VecDeque<Triangle>, odd: bool) -> bool {
    if candidates.is_empty() {
        return true;
    }

    let mut rejected = VecDeque::new();

    while !candidates.is_empty() {
        let candidate = candidates.pop_front().unwrap();

        if strip[strip.len()-1].neighbours(&candidate, odd) {
            strip.push(candidate);
            candidates.append(&mut rejected);
            if find(strip, candidates, !odd) {
                return true;
            }
            rejected.push_back(strip.pop().unwrap());
        } else if strip[strip.len()-1].neighbours(&candidate.rotate(), odd) {
            strip.push(candidate.rotate());
            candidates.append(&mut rejected);
            if find(strip, candidates, !odd) {
                return true;
            }
            rejected.push_back(strip.pop().unwrap());
        } else if strip[strip.len()-1].neighbours(&candidate.rotate().rotate(), odd) {
            strip.push(candidate.rotate().rotate());
            candidates.append(&mut rejected);
            if find(strip, candidates, !odd) {
                return true;
            }
            rejected.push_back(strip.pop().unwrap());
        } else {
            rejected.push_back(candidate);
        }
    }

    candidates.append(&mut rejected);
    false
}

fn collapse(list: Vec<Triangle>) -> Vec<u32> {
    let mut strip = vec![list[0].first, list[0].second];
    strip.append(&mut list.into_iter().map(|t| t.third).collect());

    strip
}

