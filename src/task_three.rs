use std::fs;

use crate::types::{Group, Student};

pub fn task(groups: &Vec<Group>) {
  let recommended: Vec<&Student> = groups.iter()
  .filter_map(|g| {
    if g.year == 4 {
      return Some(&g.students);
    } else {
      return None;
    }
  }).flatten().filter(|s| {
    s.average_score > 4.0
  }).collect();

  match recommended.len() {
    0 => println!("No students recommended for master degree"),
    _ => {
      println!("Recommended for master degree: {:?}", recommended);
      write_recommended_to_file(recommended);
    }
  }
}

fn write_recommended_to_file(recommended: Vec<&Student>) {
  let serialized = bincode::serialize(&recommended).unwrap();

  fs::write("master_recommended.bin", serialized).unwrap();
}
