use clap::Parser;
use reqwest::{
    blocking::Client,
    header::{self},
};
use sompp::types::*;
use std::collections::HashMap;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    access_token: String,
}

struct User {
    client: Client,
    access_token: String,
}

impl User {
    fn id(&self) -> Res<u64> {
        let resp: Students = self
            .client
            .get("https://api.somtoday.nl/rest/v1/leerlingen")
            .header(
                header::AUTHORIZATION,
                &format!("Bearer {}", self.access_token),
            )
            .header(header::ACCEPT, "application/json")
            .send()?
            .json()?;

        assert_eq!(resp.items.len(), 1);
        let student = &resp.items[0];
        assert_eq!(student.links.len(), 1);
        let link = &student.links[0];

        Ok(link.id)
    }

    fn subjects(&self) -> Res<Subjects> {
        let resp: Subjects = self
            .client
            .get("https://api.somtoday.nl/rest/v1/vakken")
            .header(
                header::AUTHORIZATION,
                &format!("Bearer {}", self.access_token),
            )
            .header(header::ACCEPT, "application/json")
            .send()?
            .json()?;

        Ok(resp)
    }

    fn grades(&self) -> Res<HashMap<String, Vec<Grade>>> {
        let mut grades = Grades { items: Vec::new() };

        let mut i = 0;
        loop {
            let mut resp: Grades = self
                .client
                .get(format!(
                    "https://api.somtoday.nl/rest/v1/resultaten/huidigVoorLeerling/{}",
                    self.id()?
                ))
                .header(
                    header::AUTHORIZATION,
                    &format!("Bearer {}", self.access_token),
                )
                .header(header::ACCEPT, "application/json")
                .header("Range", &format!("items={}-{}", i, i + 99))
                .send()?
                .json()?;

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
}

fn main() -> Res<()> {
    let args = Args::parse();
    let user = User {
        client: reqwest::blocking::Client::new(),
        access_token: args.access_token,
    };

    println!("\n########\nStarting\n########\n");

    let id = user.id()?;
    println!("id: {id}");
    let subjects = user.subjects()?;
    println!("subjects:\n{subjects:#?}");
    let _ = user.grades()?;

    Ok(())
}
