use std::{io, fs, error::Error};

mod types;
mod task_one;
mod task_two;
mod task_three;

use types::{Group, Student, Courses, Teacher, Position};


fn input_number(prompt: &str) -> usize {
  let stdio = io::stdin();
  
  loop {
    println!("{}", prompt);
    let mut input = String::new();

    match stdio.read_line(&mut input) {
      Ok(_) => {},
      Err(e) => {
          println!("Failed to read input, try again: {}", e);
          continue;
      },
    }

    match input.trim().parse::<usize>() {
        Ok(res) => return res,
        Err(e) => {
          println!("Input is not a number, try again");
          dbg!(e);
          continue;
        },
    }
  }
}

// FIX: Perfect place for generic, but it requires external crate to make generic for numbers only
fn input_float(prompt: &str) -> f32 {
  let stdio = io::stdin();
  
  loop {
    println!("{}", prompt);
    let mut input = String::new();

    match stdio.read_line(&mut input) {
      Ok(_) => {},
      Err(e) => {
          println!("Failed to read input, try again: {}", e);
          continue;
      },
    }

    match input.trim().parse::<f32>() {
        Ok(res) => return res,
        Err(_) => {
          println!("Input is not a number, try again");
          continue;
        },
    }
  }
}

fn input_string(prompt: &str) -> String {
  let stdio = io::stdin();
  
  loop {
    println!("{}", prompt);
    let mut input = String::new();

    match stdio.read_line(&mut input) {
      Ok(_) => {},
      Err(e) => {
          println!("Failed to read input, try again: {}", e);
          continue;
      },
    }

    input.truncate(input.len() - 1);
    return input;
  }
}

fn input_group(i: usize) -> Group {
  let mut g = Group{
    ..Default::default()
  };

  println!("Inputing group number {}", i + 1);

  loop {
    g.name = input_string("Input group name, like IP-52:");

    let prefix_and_code: Vec<&str> = g.name.trim().split('-').collect();
    if prefix_and_code.len() < 2 {
      println!("Bad group name format");
      continue;
    }

    if prefix_and_code[0].len() != 2 {
      println!("Bad group prefix");
      continue;
    }

    g.group_prefix = prefix_and_code[0].to_owned();

    if prefix_and_code[1].len() != 2 {
      println!("Bad group code");
      continue;
    }

    match prefix_and_code[1][..1].parse::<u8>() {
        Ok(code) => {
          g.year = 6 - code;
          break;
        },
        Err(_) => {
          println!("Bad group code");
        },
    }
  }

  let num_of_students = input_number("Input number of students:");
  g.students = input_students(num_of_students);
  g.curator = input_curator();

  g
}

fn input_students(amount: usize) -> Vec<Student> {
  let mut students = Vec::<Student>::with_capacity(amount);
  for _ in 0..amount {
    students.push(input_student());
  }

  students
}

fn input_student() -> Student {
  let mut s = Student{
    ..Default::default()
  };

  s.name.first_name = input_string("Input student\'s name:");
  s.name.last_name = input_string("Input surname:");
  s.name.middle_name = input_string("Input middle name:");
  s.average_score = input_float("Input average mark:");

  loop {
    match input_number("0 - Intramural, 1 - Extramural") {
      0 => {
        s.courses = Courses::Intramural;
        break;
      },
      1 => {
        s.courses = Courses::Extramural;
        break;
      },
      _ => println!("Wrong input, try again"),
    }
  }

  s
}

fn input_curator() -> Teacher {
  let mut c = Teacher{
    ..Default::default()
  };

  c.name.first_name = input_string("Input curator\'s name:");
  c.name.last_name = input_string("Input surname:");
  c.name.middle_name = input_string("Input middle name:");

  loop {
    match input_number("0 - Docent, 1 - Assistant, 2 - Professor") {
      0 => {
        c.position = Position::Docent;
        break;
      },
      1 => {
        c.position = Position::Assistant;
        break;
      },
      2 => {
        c.position = Position::Professor;
        break;
      }
      _ => println!("Wrong input, try again"),
    }
  }

  loop {
    let date_str = input_string("Input date of birth (DD MM YYYY):");

    let date_parts: Vec<&str> = date_str.trim().split(' ').collect();
    if date_parts.len() < 3 {
      println!("Bad date format");
      continue;
    }

    if date_parts[0].len() != 2 {
      println!("Bad day length");
      continue;
    }

    if date_parts[1].len() != 2 {
      println!("Bad month length");
      continue;
    }

    if date_parts[2].len() != 4 {
      println!("Bad year length");
      continue;
    }

    match date_parts[0].parse::<u8>() {
        Ok(day) => {
          c.day = day;
        },
        Err(_) => {
          println!("Bad day");
          continue;
        },
    }

    match date_parts[1].parse::<u8>() {
      Ok(month) => {
        c.month = month;
      },
      Err(_) => {
        println!("Bad month");
        continue;
      },
  }

    match date_parts[2].parse::<u16>() {
      Ok(year) => {
        c.year = year;
        break;
      },
      Err(e) => {
        dbg!(e);
        println!("Bad year");
        continue;
      },
    }
  }

  c
}

fn create_file() {
  let num_of_groups = input_number("Input number of groups");
  let mut groups: Vec<Group> = Vec::<Group>::with_capacity(num_of_groups);

  for i in 0..num_of_groups {
    groups.push(input_group(i));
  }

  println!("Groups: {:?}", groups);

  write_to_file(groups)
}

fn write_to_file(groups: Vec<Group>) {
  let serialized = bincode::serialize(&groups).unwrap();

  fs::write("groups.bin", serialized).unwrap();
}

fn read_file() -> Result<Vec<Group>, Box<dyn Error>> {
  let data = fs::read("groups.bin")?;

  let groups = bincode::deserialize::<Vec<Group>>(&data)?;

  Ok(groups)
}

fn open_file() {
  let groups = match read_file() {
    Ok(g) => g,
    Err(e) => {
      println!("Failed to open file: {}", e);
      return;
    },
  };

  tasks_menu(groups);
}

fn tasks_menu(groups: Vec<Group>) {
  loop {
    let selection = input_number("1.Find student's with lowest mark\n2.Old teachers\n3.Recommended for master degree\n4.Back");
    match selection {
      1 => task_one::task(&groups),
      2 => task_two::task(&groups),
      3 => task_three::task(&groups),
      4 => {
        break;
      },
      _ => {
        println!("Wrong input");
      },
    }
  }
}

fn main() {
  loop {
    let selection = input_number("1.Create file\n2.Open file\n3.Close");
    match selection {
      1 => create_file(),
      2 => open_file(),
      3 => {
        return;
      },
      _ => {
        println!("Wrong input");
      },
    }
  }
}

