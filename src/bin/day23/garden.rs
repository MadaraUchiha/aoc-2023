use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::thread;

use crate::util::grid::*;
use crate::util::point::*;

const MAX_JUNCTIONS: usize = 36;

const WALL: u8 = b'#';
const GROUND: u8 = b'.';
const SPECIAL: u8 = b'@';

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Garden {
    pub start: usize,
    pub end: usize,
    pub extra_cost: u32,
    directed: [u64; MAX_JUNCTIONS],
    undirected: [u64; MAX_JUNCTIONS],
    weights: [[u32; MAX_JUNCTIONS]; MAX_JUNCTIONS],
}

impl Garden {
    pub fn longest_distance(&self) -> u32 {
        let mut costs = [0; MAX_JUNCTIONS];
        let mut frontier = VecDeque::new();
        frontier.push_back(self.start);

        let graph = self.directed;

        while let Some(from) = frontier.pop_front() {
            let mut nodes = graph[from];

            while nodes > 0 {
                let to = nodes.trailing_zeros() as usize;
                let mask: u64 = 1 << to;
                nodes ^= mask;

                costs[to] = costs[to].max(costs[from] + self.weights[from][to]);
                frontier.push_back(to);
            }
        }

        costs[self.end] + self.extra_cost
    }

    pub fn longest_distance_undirected(&self) -> u32 {
        let shared = AtomicU32::new(0);
        let threads = thread::available_parallelism().unwrap().get();

        let mut seeds = VecDeque::new();
        seeds.push_back((self.start, 1 << self.start, 0));

        while seeds.len() < threads {
            let Some((from, visited, cost)) = seeds.pop_front() else {
                break;
            };

            if from == self.end {
                shared.fetch_max(cost, Ordering::Relaxed);
                continue;
            }

            let mut nodes = self.undirected[from] & !visited;

            while nodes > 0 {
                let to = nodes.trailing_zeros() as usize;
                let mask = 1 << to;
                nodes ^= mask;

                seeds.push_back((to, visited | mask, cost + self.weights[from][to]));
            }
        }
        thread::scope(|scope| {
            for start in &seeds {
                scope.spawn(|| worker(self, &shared, start));
            }
        });

        shared.load(Ordering::Relaxed) + self.extra_cost
    }
}

fn worker(garden: &Garden, shared: &AtomicU32, start: &(usize, u64, u32)) {
    let (from, visited, cost) = *start;
    let result = dfs(garden, from, visited);
    shared.fetch_max(result + cost, Ordering::Relaxed);
}

fn dfs(garden: &Garden, from: usize, visited: u64) -> u32 {
    if from == garden.end {
        return 0;
    }

    let mut nodes = garden.undirected[from] & !visited;
    let mut cost = 0;

    while nodes > 0 {
        let to = nodes.trailing_zeros() as usize;
        let mask = 1 << to;
        nodes ^= mask;

        cost = cost.max(garden.weights[from][to] + dfs(garden, to, visited | mask));
    }

    cost
}

impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::parse(s);
        let width = grid.width;
        let height = grid.height;

        // this avoids a bounds check
        grid[Point::new(1, 0)] = WALL;
        grid[Point::new(width - 2, height - 1)] = WALL;
        const EXTRA_BOUNDS_CHECK: u32 = 2; // removed 2 tiles

        let start = Point::new(1, 1);
        let end = Point::new(width - 2, height - 2);

        grid[start] = SPECIAL;
        grid[end] = SPECIAL;

        let mut points_of_interest = HashMap::new();
        points_of_interest.insert(start, 0);
        points_of_interest.insert(end, 1);

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let point = Point::new(x, y);
                let tile = grid[point];

                if tile == WALL {
                    continue;
                }
                let neighbors = ADJACENT
                    .iter()
                    .map(|&dir| point + dir)
                    .filter(|&p| grid[p] != WALL)
                    .count();

                if neighbors > 2 {
                    points_of_interest.insert(point, points_of_interest.len());
                    grid[point] = SPECIAL;
                }
            }
        }

        let mut frontier = VecDeque::new();
        let mut directed: [u64; 36] = [0; MAX_JUNCTIONS];
        let mut undirected: [u64; 36] = [0; MAX_JUNCTIONS];
        let mut weights: [[u32; 36]; 36] = [[0; MAX_JUNCTIONS]; MAX_JUNCTIONS];

        for (&start, &from) in &points_of_interest {
            frontier.push_back((start, 0, true));
            grid[start] = WALL; // visited

            while let Some((point, cost, forwards)) = frontier.pop_front() {
                for direction in ADJACENT {
                    let next = point + direction;
                    let tile = grid[next];
                    let next_cost = cost + 1;

                    match tile {
                        WALL => (),
                        SPECIAL => {
                            let to = points_of_interest[&next];

                            if forwards {
                                directed[from] |= 1 << to;
                            } else {
                                directed[to] |= 1 << from;
                            }

                            undirected[from] |= 1 << to;
                            undirected[to] |= 1 << from;

                            weights[from][to] = next_cost;
                            weights[to][from] = next_cost;
                        }
                        GROUND => {
                            frontier.push_back((next, next_cost, forwards));
                            grid[next] = WALL;
                        }
                        _ => {
                            let same = direction == Point::from(tile);
                            frontier.push_back((next, next_cost, forwards && same));
                            grid[next] = WALL;
                        }
                    }
                }
            }
        }

        let start = undirected[0].trailing_zeros() as usize;
        let end = undirected[1].trailing_zeros() as usize;
        let extra_cost = weights[0][start] + weights[1][end] + EXTRA_BOUNDS_CHECK;

        Ok(Self {
            start,
            end,
            extra_cost,
            directed,
            undirected,
            weights,
        })
    }
}
