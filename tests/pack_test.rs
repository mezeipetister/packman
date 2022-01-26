use packman::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[derive(Serialize, Deserialize, Clone, Default)]
struct User {
    name: String,
    age: i32,
    sex: Sex,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct User2 {
    pub sex: Sex,
    pub age: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct User3 {
    pub sex: Sex2,
    pub age: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
enum Sex {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone)]
enum Sex2 {
    Unknown,
    Male,
    Female,
}

impl Default for Sex {
    fn default() -> Self {
        Self::Male
    }
}

impl Default for Sex2 {
    fn default() -> Self {
        Self::Unknown
    }
}

#[test]
fn test_field_order_update() {
    {
        let mut u1: Pack<User> =
            Pack::load_or_init(PathBuf::from("data/pack_test"), "test_user")
                .unwrap();
        u1.update(|u| {
            u.name = "Peter".to_string();
            u.age = 33;
            u.sex = Sex::Male;
        })
        .unwrap();
    }
    let u2: Pack<User2> =
        Pack::load_or_init(PathBuf::from("data/pack_test"), "test_user")
            .unwrap();
    assert_eq!(&u2.name, "Peter");
    assert_eq!(&u2.age, &33);
    assert!(match &u2.sex {
        Sex::Male => true,
        Sex::Female => false,
    });

    let u3: Pack<User3> =
        Pack::load_or_init(PathBuf::from("data/pack_test"), "test_user")
            .unwrap();

    assert_eq!(&u3.name, "Peter");
    assert_eq!(&u3.age, &33);
    assert!(match &u3.sex {
        Sex2::Male => true,
        _ => false,
    });
}
