use crate::types::Group;

pub fn task(groups: &Vec<Group>) {
  for g in groups.iter() {
    let mut i: usize = 0;
    for j in 1..g.students.len() {
      if g.students[i].average_score > g.students[j].average_score {
        i = j;
      }
    }

    println!("Student with min avg - {}, avg - {}", g.students[i].name, g.students[i].average_score);
    println!("Group - {} Curator - {}", g.name, g.curator.name);
  }
}
