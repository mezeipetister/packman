use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use storaget::*;

#[test]
fn test_load_or_init() {
    let meaning_of_life: PackResult<Pack<i32>> =
        Pack::load_or_init(PathBuf::from("data/pack_test"), "meaning_of_life");
    assert_eq!(meaning_of_life.is_ok(), true);
}

#[test]
fn test_update() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_update",
    )
    .unwrap();
    *(meaning_of_life.as_mut()) = 17;
    &mut meaning_of_life.update(|i| *i = 42);
    assert_eq!(*(meaning_of_life), 42);
}

#[test]
fn test_as_mut() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_as_mut",
    )
    .unwrap();
    *(meaning_of_life.as_mut()) = 17;
    assert_eq!(*(meaning_of_life), 17);
    *(meaning_of_life.as_mut()) = 42;
    assert_eq!(*(meaning_of_life), 42);
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Car {
    fuel: String,
    number_of_seats: u32,
}

#[test]
fn test_as_ref() {
    let mut meaning_of_life: Pack<Car> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_as_ref",
    )
    .unwrap();
    meaning_of_life.as_mut().unpack().number_of_seats = 4;
    assert_eq!(get_seats(&meaning_of_life), 4);
}

fn get_seats(car: &Pack<Car>) -> u32 {
    car.unpack().number_of_seats
}

#[test]
fn test_as_deref() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_deref",
    )
    .unwrap();
    *(meaning_of_life.as_mut()) = 42;

    // Manually drop
    // meaning_of_life variable
    drop(meaning_of_life);

    // Init it again
    // and read the stored value
    let meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_deref",
    )
    .unwrap();

    assert_eq!(*(meaning_of_life), 42);
}

#[test]
fn test_vector() {
    let mut meaning_of_life: Pack<Vec<i32>> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_vec",
    )
    .unwrap();
    (meaning_of_life.as_mut()).push(1);
    (meaning_of_life.as_mut()).push(2);
    (meaning_of_life.as_mut()).push(3);
    assert_eq!(*(meaning_of_life), vec![1, 2, 3]);
}

#[test]
fn test_struct() {
    #[derive(Serialize, Deserialize, Clone, Default)]
    struct Car {
        fuel: String,
        number_of_seats: u32,
    }

    let mut meaning_of_life: Pack<Car> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_struct",
    )
    .unwrap();
    *(meaning_of_life.as_mut()) = Car {
        fuel: "electric".to_string(),
        number_of_seats: 6,
    };
    assert_eq!(*(meaning_of_life).fuel, "electric".to_string());
}

#[test]
fn test_get() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_get",
    )
    .unwrap();
    *(meaning_of_life.as_mut()) = 42;
    assert_eq!(meaning_of_life.get(|i| i.clone()), 42);
}

#[test]
fn test_map() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_map",
    )
    .unwrap();
    *(meaning_of_life.as_mut()) = 42;
    assert_eq!(meaning_of_life.map(|i| i * 2), 84);
}

#[test]
fn test_update_iter_1000() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_update_iter_1000",
    )
    .unwrap();
    for _ in 0..1000 {
        *(meaning_of_life.as_mut()) += 1;
    }
    assert_eq!(meaning_of_life.get(|i| i.clone()), 1000);
}

#[test]
fn test_update_iter_10000() {
    let mut meaning_of_life: Pack<i32> = Pack::load_or_init(
        PathBuf::from("data/pack_test"),
        "meaning_of_life_update_iter_10000",
    )
    .unwrap();
    for _ in 0..10000 {
        *(meaning_of_life.as_mut()) += 1;
    }
    assert_eq!(meaning_of_life.get(|i| i.clone()), 10000);
}
