use sompp::{types::*, Res, tools::*};
use chrono::NaiveDate;
use wry::application::event::Event;
use wry::application::event::WindowEvent;
use wry::application::event_loop::ControlFlow;
use wry::application::event_loop::EventLoop;
use wry::application::platform::run_return::EventLoopExtRunReturn;
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

static mut CODE: Option<String> = None;

fn main() -> Res<()> {
    let user_url = UserData::url();

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Som++")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
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
        .with_url(&user_url.url)?
        .build()?;

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

    let user_data = UserData::with_code(code, &user_url.verifier)?;
    let user = User::new(user_data)?;

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
