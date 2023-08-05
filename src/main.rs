use sompp_tools::{types::*, Res};
use chrono::NaiveDate;

fn main() -> Res<()> {
    let user_data = UserData::new()?;
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
