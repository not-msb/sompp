use anyhow::Result;
use sompp_tools::{types::UserData, tools::between};
#[cfg(target_os = "android")]
use wry::android_binding;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
        window::{Window, WindowBuilder},
        platform::run_return::EventLoopExtRunReturn,
    },
    webview::{WebView, WebViewBuilder},
};

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("somtomorrow"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    env_logger::init();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn _start_app() {
    stop_unwind(|| main().unwrap());
}

#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    android_binding!(com_dupunkto, somtomorrow, _start_app);
    #[cfg(target_os = "ios")]
    _start_app()
}

#[no_mangle]
#[inline(never)]
#[cfg(target_os = "linux")]
pub extern "C" fn start_app() -> Result<()> {
    main()
}

static mut CODE: Option<String> = None;

pub fn main() -> Result<()> {
    init_logging();
    let user_url = UserData::url();

    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Somtomorrow")
        .build(&event_loop)?;

    let webview = WebViewBuilder::new(window)?
        .with_url(&user_url.url)?
        .with_navigation_handler(|uri| -> bool {
            if uri.starts_with("somtodayleerling://") {
                unsafe {
                    CODE = Some(between(&uri, "code=", "&"));
                }
                false
            } else {
                true
            }
        })
        .with_ipc_handler(|_, s| {
            dbg!(s);
        })
        .build()?;

    let _ = event_loop.run_return(|event, _, control_flow| {
        *control_flow = unsafe {
            match &CODE {
                Some(_) => ControlFlow::Exit,
                None => ControlFlow::Wait,
            }
        };

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested { .. },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    webview.load_url("https://google.com");

    let _code = unsafe {
        match &CODE {
            Some(c) => c,
            None => return Ok(()),
        }
    };

    Ok(())
}
