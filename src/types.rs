use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct Student {
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Subject {
    #[serde(rename = "naam")]
    pub name: String,
    #[serde(rename = "afkorting")]
    pub acronym: String,
}

#[derive(Debug, Deserialize)]
pub struct Grade {
    #[serde(rename = "resultaat")]
    pub result: Option<String>,
    #[serde(rename = "omschrijving")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(rename = "vak")]
    pub subject: Subject,
}

#[derive(Debug, Deserialize)]
pub struct Appointment {
    #[serde(rename = "titel")]
    pub title: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Homework {
    #[serde(rename = "studiewijzerItem")]
    pub studiewijzer_item: StudiewijzerItem,
    #[serde(rename = "lesgroep")]
    pub lesson_group: LessonGroup,
    #[serde(rename = "datumTijd")]
    pub date_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct LessonGroup {
    #[serde(rename = "vak")]
    pub subject: Subject,
}

#[derive(Debug, Deserialize)]
pub struct StudiewijzerItem {
    #[serde(rename = "onderwerp")]
    pub topic: String,
    #[serde(rename = "omschrijving")]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Students {
    pub items: Vec<Student>,
}

#[derive(Debug, Deserialize)]
pub struct Subjects {
    pub items: Vec<Subject>,
}

#[derive(Debug, Deserialize)]
pub struct Grades {
    pub items: Vec<Grade>,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub items: Vec<Appointment>,
}

#[derive(Debug, Deserialize)]
pub struct MultHomework {
    pub items: Vec<Homework>,
}
