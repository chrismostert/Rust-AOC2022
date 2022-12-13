use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Val(usize),
    List(Vec<Self>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_tokens(tokens: &mut Vec<&str>) -> Vec<Packet> {
            let mut result = Vec::new();
            while let Some(token) = tokens.pop() {
                match token {
                    "]" => {
                        return result;
                    }
                    "[" => {
                        result.push(Packet::List(parse_tokens(tokens)));
                    }
                    "" => {}
                    val => {
                        result.push(Packet::Val(val.parse().unwrap()));
                    }
                };
            }
            result
        }

        let tokenstring = s.replace('[', "[,").replace(']', ",]");
        let mut tokens: Vec<_> = tokenstring.split(',').rev().collect();
        Ok(Packet::List(parse_tokens(&mut tokens)))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Val(a), Self::Val(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => {
                let (mut a, mut b) = (a.iter(), b.iter());
                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => match a.partial_cmp(b) {
                            Some(Ordering::Equal) => {}
                            Some(order) => return Some(order),
                            None => unreachable!(),
                        },
                        (None, Some(_)) => return Some(Ordering::Less),
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, None) => return Some(Ordering::Equal),
                    }
                }
            }
            (Self::Val(a), Self::List(_)) => Self::List(vec![Self::Val(*a)]).partial_cmp(other),
            (Self::List(_), Self::Val(b)) => self.partial_cmp(&Self::List(vec![Self::Val(*b)])),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let packets: Vec<_> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.parse::<Packet>().unwrap())
        .collect();

    let p1 = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, chunk)| chunk[0] < chunk[1])
        .map(|(idx, _)| idx + 1)
        .sum::<usize>();

    let p2 = packets.iter().fold((1, 2), |acc, packet| {
        (
            acc.0 + (packet < &"[[2]]".parse::<Packet>().unwrap()) as usize,
            acc.1 + (packet < &"[[6]]".parse::<Packet>().unwrap()) as usize,
        )
    });

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2.0 * p2.1);
}
