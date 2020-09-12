use packman::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub trait E {
  fn e(&self) -> String;
}

impl E for &'static str {
  fn e(&self) -> String {
    self.to_string()
  }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Car {
  pub id: String,
  pub name: String,
  pub hp: u32,
}

impl Car {
  pub fn new(id: String, name: String, hp: u32) -> Self {
    Car { id, name, hp }
  }
}

impl VecPackMember for Car {
  type Out = str;
  fn get_id(&self) -> &str {
    &self.id
  }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Robot {
  pub id: u32,
  pub name: String,
  pub can_speak: bool,
}

impl Robot {
  pub fn new(id: u32, name: String, can_speak: bool) -> Self {
    Robot {
      id,
      name,
      can_speak,
    }
  }
}

impl VecPackMember for Robot {
  type Out = u32;
  fn get_id(&self) -> &u32 {
    &self.id
  }
}

#[test]
fn test_vecpack_load_or_init() {
  let meaning_of_life: PackResult<VecPack<Car>> =
    VecPack::load_or_init(PathBuf::from("data/vecpack_test_load_or_init"));
  assert_eq!(meaning_of_life.is_ok(), true);
  assert_eq!((*meaning_of_life.unwrap()).len(), 0);
}

fn create_dummy_vecpack(path: PathBuf) -> VecPack<Car> {
  let mut meaning_of_life: VecPack<Car> = VecPack::load_or_init(path).unwrap();
  meaning_of_life
    .insert(Car::new("1".to_string(), "CarSmall".to_string(), 150))
    .unwrap();
  meaning_of_life
    .insert(Car::new("2".to_string(), "CarBig".to_string(), 650))
    .unwrap();
  meaning_of_life
    .insert(Car::new("3".to_string(), "CarMedium".to_string(), 250))
    .unwrap();
  meaning_of_life
}

#[test]
fn test_vecpack_insert() {
  let mut meaning_of_life: VecPack<Car> =
    VecPack::load_or_init(PathBuf::from("data/vecpack_test_insert")).unwrap();
  meaning_of_life
    .insert(Car::new("1".to_string(), "CarSmall".to_string(), 150))
    .unwrap();
  meaning_of_life
    .insert(Car::new("2".to_string(), "CarBig".to_string(), 650))
    .unwrap();
  meaning_of_life
    .insert(Car::new("3".to_string(), "CarMedium".to_string(), 250))
    .unwrap();

  assert_eq!((*meaning_of_life).len(), 3);
}

#[test]
fn test_vecpack_as_mut() {
  let mut cars =
    create_dummy_vecpack(PathBuf::from("data/vecpack_test_as_mut"));
  cars.into_iter().for_each(|i| i.as_mut().hp = 1);
  assert_eq!(cars.get(0).unwrap().hp, 1);
}

#[test]
fn test_vecpack_find_id() {
  let cars = create_dummy_vecpack(PathBuf::from("data/vecpack_test_find_id"));
  assert_eq!(cars.find_id("3").is_ok(), true);
  assert_eq!(cars.find_id("1").unwrap().unpack().hp, 150);
  assert_eq!(cars.find_id("2").unwrap().unpack().hp, 650);
  assert_eq!(cars.find_id("3").unwrap().unpack().hp, 250);
}

#[test]
fn test_vecpack_find_id_mut_update() {
  let mut cars =
    create_dummy_vecpack(PathBuf::from("data/vecpack_test_find_id_mut_update"));
  cars.find_id_mut("1").unwrap().update(|i| i.hp = 1).unwrap();
  cars
    .find_id_mut("2")
    .unwrap()
    .update(|i| i.hp = 11)
    .unwrap();
  cars
    .find_id_mut("3")
    .unwrap()
    .update(|i| i.hp = 111)
    .unwrap();

  assert_eq!(cars.find_id("1").unwrap().hp, 1);
  assert_eq!(cars.find_id("2").unwrap().hp, 11);
  assert_eq!(cars.find_id("3").unwrap().hp, 111);
}

#[test]
fn test_vecpack_find_id_mut_as_mut() {
  let mut cars =
    create_dummy_vecpack(PathBuf::from("data/vecpack_test_find_id_mut_as_mut"));
  cars.find_id_mut("1").unwrap().as_mut().hp = 1;
  cars.find_id_mut("2").unwrap().as_mut().hp = 11;
  cars.find_id_mut("3").unwrap().as_mut().hp = 111;
  assert_eq!(cars.find_id_mut("4").is_ok(), false);
  assert_eq!(cars.find_id_mut("100").is_ok(), false);

  assert_eq!(cars.find_id("1").unwrap().hp, 1);
  assert_eq!(cars.find_id("2").unwrap().hp, 11);
  assert_eq!(cars.find_id("3").unwrap().hp, 111);
}

#[test]
fn test_id_str() {
  let mut robots: VecPack<Robot> =
    VecPack::load_or_init(PathBuf::from("data/vecpack_test_id_str")).unwrap();

  robots
    .insert(Robot::new(1, "robot_a".to_string(), true))
    .unwrap();
  robots
    .insert(Robot::new(2, "robot_b".to_string(), false))
    .unwrap();
  robots
    .insert(Robot::new(3, "robot_c".to_string(), false))
    .unwrap();

  assert_eq!(robots.find_id(&2).is_ok(), true);
}

#[test]
fn test_mut_ref() {
  let mut robots: VecPack<Robot> =
    VecPack::load_or_init(PathBuf::from("data/vecpack_test_mut_red")).unwrap();
  robots
    .insert(Robot::new(1, "Mini Robot".to_string(), true))
    .unwrap();

  let robots = robots.as_vec_mut();
  robots.get_mut(0).unwrap().as_mut().name = "Mini Roboto".to_string();
  assert_eq!(robots.get(0).unwrap().name, "Mini Roboto");
}
