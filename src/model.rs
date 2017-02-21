use rand::{self, Rand, Rng};
use std::fmt;

pub trait ModelIterator: Iterator<Item = Model> { }
impl<T: Iterator<Item = Model>> ModelIterator for T { }

#[derive(Debug)]
pub struct Model {
    pub header: Header,
    pub locations: Vec<Location>,
}

#[derive(Debug)]
pub struct Header {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Location {
    pub address: String,
    pub transactions: Vec<Transaction>
}

#[derive(Debug)]
pub struct Transaction {
    pub amount: i64,
    pub method: Method,
}

#[derive(Debug)]
pub enum Method {
    Credit,
    Debit,
    Cash,
    Theft,
    Barter,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Method::Credit => f.pad("credit"),
            Method::Debit => f.pad("debit"),
            Method::Cash => f.pad("cash"),
            Method::Theft => f.pad("theft"),
            Method::Barter => f.pad("barter"),
        }
    }
}

impl Rand for Method {
    fn rand<R: Rng>(r: &mut R) -> Self {
        match r.gen_range(0, 5) {
            0 => Method::Credit,
            1 => Method::Debit,
            2 => Method::Cash,
            3 => Method::Theft,
            _ => Method::Barter,
        }
    }
}

#[derive(Default)]
pub struct ModelGenerator(i64);

static NAMES: &'static [&'static str] = &[
    "Dunneville",
    "Manneville",
    "Bonneville",
    "Ninneville",
    "Terneville",
    "Werneville",
    "Qoxxeville",
];

impl Iterator for ModelGenerator {
    type Item = Model;

    fn next(&mut self) -> Option<Model> {
        let id = self.0;
        self.0 += 1;

        Some(Model {
            header: Header {
                id: id.to_string(),
                name: rand::thread_rng().choose(NAMES).unwrap().to_string(),
            },
            locations: (0..(rand::thread_rng().gen_range(1, 13)))
                .map(|_| Location {
                        address: format!("{} {}", rand::thread_rng().choose(NAMES).unwrap(), rand::thread_rng().gen_range(100, 501)),
                        transactions: (0..(rand::thread_rng().gen_range(1, 8)))
                            .map(|_| Transaction { amount: rand::thread_rng().gen_range(489, 11589), method: rand::thread_rng().gen() })
                            .collect(),
                })
                .collect(),
        })
    }
}
