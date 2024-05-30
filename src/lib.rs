pub mod app;
pub mod ui;

pub use app::TypingApp;
pub fn run() -> iced::Result {
    TypingApp::run()
}