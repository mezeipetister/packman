use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CarV0 {
    id: usize,
    hp: u32,
    number_of_seats: u32,
    color: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CarV1 {
    id: usize,
    hpower: u32,
    seats_number: u32,
    color: String,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CarV2 {
    id: usize,
    horsepower: u32,
    seats_number: u32,
    car_color: String,
}

impl From<CarV0> for CarV1 {
    fn from(from: CarV0) -> Self {
        CarV1 {
            id: from.id,
            hpower: from.hp,
            seats_number: from.number_of_seats,
            color: from.color,
        }
    }
}

impl From<CarV1> for CarV2 {
    fn from(from: CarV1) -> Self {
        CarV2 {
            id: from.id,
            horsepower: from.hpower,
            seats_number: from.seats_number,
            car_color: from.color,
        }
    }
}

impl TryFrom for CarV0 {
    type TryFrom = CarV0;
}

impl TryFrom for CarV1 {
    type TryFrom = CarV0;
}

impl TryFrom for CarV2 {
    type TryFrom = CarV1;
}

#[test]
fn test_try_load_or_init() {
    let mut meaning_of_life: Pack<CarV0> = Pack::try_load_or_init(
        PathBuf::from("data/pack_try_from_test"),
        "meaning_of_life",
    )
    .unwrap();
    meaning_of_life.as_mut().number_of_seats = 4;
    assert_eq!(meaning_of_life.id, 0);
    let meaning_of_life_v1: Pack<CarV1> = Pack::try_load_or_init(
        PathBuf::from("data/pack_try_from_test"),
        "meaning_of_life",
    )
    .unwrap();
    assert_eq!(meaning_of_life_v1.seats_number, 4);
    let meaning_of_life_v2: Pack<CarV2> = Pack::try_load_or_init(
        PathBuf::from("data/pack_try_from_test"),
        "meaning_of_life",
    )
    .unwrap();
    assert_eq!(meaning_of_life_v2.seats_number, 4);
}
