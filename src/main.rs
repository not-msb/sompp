use chrono::NaiveDate;
use reqwest::{
    blocking::Client,
    header::{self, HeaderMap, HeaderValue},
};
use sompp::{types::*, Res};
use std::collections::HashMap;
use wry::application::event::Event;
use wry::application::event::WindowEvent;
use wry::application::event_loop::ControlFlow;
use wry::application::event_loop::EventLoop;
use wry::application::platform::run_return::EventLoopExtRunReturn;
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

struct User {
    client: Client,
    headers: HeaderMap,
}

impl User {
    fn new(access_token: String) -> Res<User> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {access_token}")).unwrap(),
        );
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let user = User {
            client: reqwest::blocking::Client::new(),
            headers,
        };

        Ok(user)
    }

    fn id(&self) -> Res<usize> {
        let resp: Students = self
            .client
            .get("https://api.somtoday.nl/rest/v1/leerlingen")
            .headers(self.headers.clone())
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
            .headers(self.headers.clone())
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
                .headers(self.headers.clone())
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

    fn schedule(&self, begin: NaiveDate, end: NaiveDate) -> Res<Schedule> {
        let resp: Schedule = self
            .client
            .get("https://api.somtoday.nl/rest/v1/afspraken")
            .headers(self.headers.clone())
            .query(&[
                ("begindatum", &format!("{}", begin.format("%Y-%m-%d"))),
                ("einddatum", &format!("{}", end.format("%Y-%m-%d"))),
            ])
            .send()?
            .json()?;

        Ok(resp)
    }

    fn homework_appointments(&self, begin: NaiveDate, end: NaiveDate) -> Res<MultHomework> {
        let resp: MultHomework = self
            .client
            .get("https://api.somtoday.nl/rest/v1/studiewijzeritemafspraaktoekenningen")
            .headers(self.headers.clone())
            .query(&[("begintNaOfOp", &format!("{}", begin.format("%Y-%m-%d")))])
            .send()?
            .json()?;

        let filtered: Vec<Homework> = resp
            .items
            .into_iter()
            .filter(|item| item.date_time.date_naive() < end)
            .collect();

        Ok(MultHomework { items: filtered })
    }

    fn homework_days(&self, begin: NaiveDate, end: NaiveDate) -> Res<MultHomework> {
        let resp: MultHomework = self
            .client
            .get("https://api.somtoday.nl/rest/v1/studiewijzeritemdagtoekenningen")
            .headers(self.headers.clone())
            .query(&[("begintNaOfOp", &format!("{}", begin.format("%Y-%m-%d")))])
            .send()?
            .json()?;

        let filtered: Vec<Homework> = resp
            .items
            .into_iter()
            .filter(|item| item.date_time.date_naive() < end)
            .collect();

        Ok(MultHomework { items: filtered })
    }
}

// TMP

static mut CODE: Option<String> = None;

fn between(input: &str, prefix: &str, postfix: &str) -> String {
    let mut b = 0;
    while !input[b..].starts_with(prefix) {
        b += 1;
    }
    b += prefix.len();

    let mut e = b;
    while !input[e..].starts_with(postfix) {
        e += 1;
    }
    e += prefix.len();

    input[b..e].to_owned()
}

// END TMP

fn main() -> Res<()> {
    let url = UserData::url();

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Som++")
        .build(&event_loop)?;
    let webview = WebViewBuilder::new(window)?
        .with_navigation_handler(|uri| -> bool {
            if uri.starts_with("somtodayleerling://") {
                println!("found SomToday scheme!");
                unsafe {
                    CODE = Some(between(&uri, "code=", "&"));
                }
                false
            } else {
                true
            }
        })
        .with_url(&url)?
        .build()?;

    webview.open_devtools();

    let _ = event_loop.run_return(|event, _, control_flow| {
        *control_flow = unsafe {
            match &CODE {
                Some(_) => ControlFlow::Exit,
                None => ControlFlow::Wait,
            }
        };

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });

    let code = unsafe {
        match &CODE {
            Some(c) => c,
            None => todo!("Return some error"),
        }
    };

    let user_data = UserData::with_code(code)?;
    let user = User::new(user_data.access_token)?;

    println!("\n########\nStarting\n########\n");

    let id = user.id()?;
    println!("id: {id}");

    let subjects = user.subjects()?;
    println!("subjects:\n{subjects:#?}");

    let grades = user.grades()?;
    println!("grades:\n{grades:#?}");

    let schedule = user.schedule(
        NaiveDate::from_ymd_opt(2023, 6, 12).unwrap(),
        NaiveDate::from_ymd_opt(2023, 6, 17).unwrap(),
    )?;
    println!("schedule:\n{schedule:#?}");

    let hw_appointments = user.homework_appointments(
        NaiveDate::from_ymd_opt(2023, 6, 12).unwrap(),
        NaiveDate::from_ymd_opt(2023, 6, 17).unwrap(),
    )?;
    println!("hw_appointments:\n{hw_appointments:#?}");

    let hw_days = user.homework_days(
        NaiveDate::from_ymd_opt(2023, 6, 12).unwrap(),
        NaiveDate::from_ymd_opt(2023, 6, 17).unwrap(),
    )?;
    println!("hw_days:\n{hw_days:#?}");

    Ok(())
}
