use crate::{tools::*, Res};
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use std::io::stdin;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub data: UserData,
}

impl User {
    pub fn new(data: UserData) -> Res<User> {
        let user = User {
            data,
        };

        Ok(user)
    }

    pub fn id(&self) -> Res<usize> {
        let resp: Students = ureq::get("https://api.somtoday.nl/rest/v1/leerlingen")
            .set("AUTHORIZATION", &format!("Bearer {}", self.data.access_token))
            .set("ACCEPT", "application/json")
            .call()?
            .into_json()?;

        assert_eq!(resp.items.len(), 1);
        let student = &resp.items[0];
        assert_eq!(student.links.len(), 1);
        let link = &student.links[0];

        Ok(link.id)
    }

    pub fn subjects(&self) -> Res<Subjects> {
        let resp: Subjects = ureq::get("https://api.somtoday.nl/rest/v1/vakken")
            .set("AUTHORIZATION", &format!("Bearer {}", self.data.access_token))
            .set("ACCEPT", "application/json")
            .call()?
            .into_json()?;

        Ok(resp)
    }

    pub fn grades(&self) -> Res<HashMap<String, Vec<Grade>>> {
        let mut grades = Grades { items: Vec::new() };

        let mut i = 0;
        loop {
            let mut resp: Grades = ureq::get(&format!(
                    "https://api.somtoday.nl/rest/v1/resultaten/huidigVoorLeerling/{}",
                    self.id()?
                ))
                .set("AUTHORIZATION", &format!("Bearer {}", self.data.access_token))
                .set("ACCEPT", "application/json")
                .set("Range", &format!("items={}-{}", i, i + 99))
                .call()?
                .into_json()?;

            let length = resp.items.len();
            grades.items.append(&mut resp.items);
            if length < 100 {
                break;
            }

            i += 100;
        }

        let mut subjects: HashMap<String, Vec<Grade>> = HashMap::new();
        for subject in self.subjects()?.items {
            subjects.insert(subject.acronym, Vec::new());
        }

        for grade in grades.items {
            match subjects.get_mut(&grade.subject.acronym) {
                Some(subject) => subject.push(grade),
                None => unreachable!(),
            }
        }

        Ok(subjects)
    }

    pub fn schedule(&self, begin: NaiveDate, end: NaiveDate) -> Res<Schedule> {
        let resp: Schedule = ureq::get("https://api.somtoday.nl/rest/v1/afspraken")
            .set("AUTHORIZATION", &format!("Bearer {}", self.data.access_token))
            .set("ACCEPT", "application/json")
            .query("begindatum", &format!("{}", begin.format("%Y-%m-%d")))
            .query("einddatum", &format!("{}", end.format("%Y-%m-%d")))
            .call()?
            .into_json()?;

        Ok(resp)
    }

    pub fn homework_appointments(&self, begin: NaiveDate, end: NaiveDate) -> Res<MultHomework> {
        let resp: MultHomework = ureq::get("https://api.somtoday.nl/rest/v1/studiewijzeritemafspraaktoekenningen")
            .set("AUTHORIZATION", &format!("Bearer {}", self.data.access_token))
            .set("ACCEPT", "application/json")
            .query("begintNaOfOp", &format!("{}", begin.format("%Y-%m-%d")))
            .call()?
            .into_json()?;

        let filtered: Vec<Homework> = resp
            .items
            .into_iter()
            .filter(|item| item.date_time.date_naive() < end)
            .collect();

        Ok(MultHomework { items: filtered })
    }

    pub fn homework_days(&self, begin: NaiveDate, end: NaiveDate) -> Res<MultHomework> {
        let resp: MultHomework = ureq::get("https://api.somtoday.nl/rest/v1/studiewijzeritemdagtoekenningen")
            .set("AUTHORIZATION", &format!("Bearer {}", self.data.access_token))
            .set("ACCEPT", "application/json")
            .query("begintNaOfOp", &format!("{}", begin.format("%Y-%m-%d")))
            .call()?
            .into_json()?;

        let filtered: Vec<Homework> = resp
            .items
            .into_iter()
            .filter(|item| item.date_time.date_naive() < end)
            .collect();

        Ok(MultHomework { items: filtered })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUrl {
    pub url: String,
    pub verifier: String,
    pub challenge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub access_token: String,
    pub refresh_token: String,
    pub somtoday_api_url: String,
    pub somtoday_oop_url: String,
    pub id_token: String,
}

impl UserData {
    pub fn url() -> UserUrl {
        const REDIRECT_URI: &str = "somtodayleerling://oauth/callback";
        const SCHOOL_UUID: &str = "ec284fda-3d0f-4f54-a77e-91d94b94ff1a";
        const CLIENT_ID: &str = "D50E0C06-32D1-4B41-A137-A9A850C892C2";

        let verifier = url_encode(&random_bytes(32));
        let challenge = url_encode(&sha256(verifier.as_bytes()));

        UserUrl {
            url: format!("https://inloggen.somtoday.nl/oauth2/authorize?response_type=code&prompt=login&redirect_uri={REDIRECT_URI}&client_id={CLIENT_ID}&state=sompp&response_type=code&scope=openid&tenant_uuid={SCHOOL_UUID}&session=no_session&code_challenge_method=S256&code_challenge={challenge}"),
            verifier,
            challenge
         }
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

        let resp = ureq::post("https://inloggen.somtoday.nl/oauth2/token")
            .set("Content-Type", "application/x-www-form-urlencoded")
            .query("grant_type", "authorization_code")
            .query("redirect_uri", REDIRECT_URI)
            .query("code_verifier", &verifier)
            .query("code", &code)
            .query("scope", "openid")
            .query("client_id", CLIENT_ID)
            .call()?
            .into_json()?;

        println!("Resp: {resp:#?}");

        Ok(resp)
    }

    pub fn with_code(code: &str, verifier: &str) -> Res<UserData> {
        const REDIRECT_URI: &str = "somtodayleerling://oauth/callback";
        const CLIENT_ID: &str = "D50E0C06-32D1-4B41-A137-A9A850C892C2";

        let resp = ureq::post("https://inloggen.somtoday.nl/oauth2/token")
            .set("Content-Type", "application/x-www-form-urlencoded")
            .query("grant_type", "authorization_code")
            .query("redirect_uri", REDIRECT_URI)
            .query("code_verifier", verifier)
            .query("code", code)
            .query("scope", "openid")
            .query("client_id", CLIENT_ID)
            .call()?
            .into_json()?;

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    #[serde(rename = "naam")]
    pub name: String,
    #[serde(rename = "afkorting")]
    pub acronym: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Appointment {
    #[serde(rename = "titel")]
    pub title: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Homework {
    #[serde(rename = "studiewijzerItem")]
    pub studiewijzer_item: StudiewijzerItem,
    #[serde(rename = "lesgroep")]
    pub lesson_group: LessonGroup,
    #[serde(rename = "datumTijd")]
    pub date_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LessonGroup {
    #[serde(rename = "vak")]
    pub subject: Subject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudiewijzerItem {
    #[serde(rename = "onderwerp")]
    pub topic: String,
    #[serde(rename = "omschrijving")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Students {
    pub items: Vec<Student>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subjects {
    pub items: Vec<Subject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grades {
    pub items: Vec<Grade>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub items: Vec<Appointment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultHomework {
    pub items: Vec<Homework>,
}
