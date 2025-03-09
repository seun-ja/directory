use chrono::NaiveDateTime;
use uuid::Uuid;

pub mod school_directory_d;
pub mod teahers_d;

pub struct SchoolDirectorySchema {
    pub id: Uuid,
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
    pub subjects_taught: Option<Vec<String>>,
    pub government_approved: Option<bool>,
    pub awards_recognition: Option<Vec<String>>,
    pub management_board: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct TeacherSchema {
    pub id: Uuid,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
