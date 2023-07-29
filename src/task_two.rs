use crate::types::{Group, Teacher, Position};

pub fn task(groups: &Vec<Group>) {
  let old_teachers: Vec<Teacher> = groups.iter().map(|g| {
    g.curator.clone()
  }).filter(|c| {
    c.position == Position::Professor && 2016 - c.year > 60
  }).collect();

  match old_teachers.len() {
    0 => println!("No old teachers"),
    _ => {
      println!("Old teachers:");
      old_teachers.iter().for_each(|t| {
        println!("Name: {} Age: {}", t.name, 2016 - t.year);
      });
    }
  }
}
