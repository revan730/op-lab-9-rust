use core::fmt::Display;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum Position {
    #[default]
    Docent,
    Assistant,
    Professor
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum Courses {
    #[default]
    Intramural,
    Extramural,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct FullName {
    pub last_name: String,
    pub middle_name: String,
    pub first_name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Teacher {
    pub name: FullName,
    pub month: u8,
    pub day: u8,
    pub year: u16,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Student {
    pub name: FullName,
    pub average_score: f32,
    pub courses: Courses,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Group {
    pub students: Vec<Student>,
    pub curator: Teacher,
    pub name: String,
    pub year: u8,
    pub group_prefix: String, 
}

impl Display for FullName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.first_name, self.middle_name, self.last_name)
    }
}
