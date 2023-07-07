use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub id: u64,
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
