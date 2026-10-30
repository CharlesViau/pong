mod desktop_app;
mod game;

use desktop_app::DesktopApp;

fn main() {
    DesktopApp::build().run();
}
