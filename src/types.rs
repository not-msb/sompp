use crate::{tools::*, Res};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::io::stdin;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub access_token: String,
    pub refresh_token: String,
    pub somtoday_api_url: String,
    pub somtoday_oop_url: String,
    pub id_token: String,
}

impl UserData {
    pub fn url() -> (String, String) {
        const REDIRECT_URI: &str = "somtodayleerling://oauth/callback";
        const SCHOOL_UUID: &str = "ec284fda-3d0f-4f54-a77e-91d94b94ff1a";
        const CLIENT_ID: &str = "D50E0C06-32D1-4B41-A137-A9A850C892C2";

        let verifier = url_encode(&random_bytes(32));
        let challenge = url_encode(&sha256(verifier.as_bytes()));

        (format!("https://inloggen.somtoday.nl/oauth2/authorize?response_type=code&prompt=login&redirect_uri={REDIRECT_URI}&client_id={CLIENT_ID}&state=sompp&response_type=code&scope=openid&tenant_uuid={SCHOOL_UUID}&session=no_session&code_challenge_method=S256&code_challenge={challenge}"), verifier)
    }

    pub fn new() -> Res<UserData> {
        const REDIRECT_URI: &str = "somtodayleerling://oauth/callback";
        const SCHOOL_UUID: &str = "ec284fda-3d0f-4f54-a77e-91d94b94ff1a";
        const CLIENT_ID: &str = "D50E0C06-32D1-4B41-A137-A9A850C892C2";

        let verifier = url_encode(&random_bytes(32));
        let challenge = url_encode(&sha256(verifier.as_bytes()));

        let url = format!(
        "https://inloggen.somtoday.nl/oauth2/authorize?response_type=code&prompt=login&redirect_uri={REDIRECT_URI}&client_id={CLIENT_ID}&state=sompp&response_type=code&scope=openid&tenant_uuid={SCHOOL_UUID}&session=no_session&code_challenge_method=S256&code_challenge={challenge}"
        );

        let _ = Command::new("brave-bin").args(&[url]).output().unwrap();

        let mut code = String::new();
        println!("Enter code:");
        stdin().read_line(&mut code).unwrap();

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post("https://inloggen.somtoday.nl/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .query(&[
                ("grant_type", "authorization_code"),
                ("redirect_uri", REDIRECT_URI),
                ("code_verifier", &verifier),
                ("code", &code),
                ("scope", "openid"),
                ("client_id", CLIENT_ID),
            ])
            .send()?
            .json()?;

        Ok(resp)
    }

    pub fn with_code(code: &str, verifier: &str) -> Res<UserData> {
        const REDIRECT_URI: &str = "somtodayleerling://oauth/callback";
        const CLIENT_ID: &str = "D50E0C06-32D1-4B41-A137-A9A850C892C2";

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post("https://inloggen.somtoday.nl/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .query(&[
                ("grant_type", "authorization_code"),
                ("redirect_uri", REDIRECT_URI),
                ("code_verifier", verifier),
                ("code", code),
                ("scope", "openid"),
                ("client_id", CLIENT_ID),
            ])
            .send()?
            .text()?;

        todo!("{resp}")
    }
}

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
