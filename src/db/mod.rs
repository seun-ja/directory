use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::services::search::{Searchable, SearchableTerm};

pub mod school_directory_d;
pub mod teachers_d;

#[derive(Debug, Serialize)]
pub struct SchoolDirectorySchema {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub state: Option<String>,
    pub local_government: Option<String>,
    pub postal_code: Option<String>,
    pub website: Option<String>,
    pub about: Option<String>,
    pub current_population: Option<i32>,
    pub staff_strength: Option<i32>,
    pub year_established: Option<i32>,
    pub curriculum_offered: Option<String>,
    pub subjects_taught: Vec<String>,
    pub government_approved: Option<bool>,
    pub awards_recognition: Vec<String>,
    pub management_board: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Searchable for SchoolDirectorySchema {}

impl SearchableTerm for SchoolDirectorySchema {
    fn search_term(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Serialize)]
pub struct TeacherSchema {
    pub id: String,
    pub trcn: Option<String>,
    pub name: String,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub state: Option<String>,
    pub local_government: Option<String>,
    pub qualifications: Option<Vec<String>>,
    pub experience: Option<Vec<serde_json::Value>>, // TODO: create a struct for it
    pub speciality: Option<String>,
    pub status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Searchable for TeacherSchema {}
impl SearchableTerm for TeacherSchema {
    fn search_term(&self) -> String {
        self.name.clone()
    }
}
