#![windows_subsystem = "windows"]

use druid::widget::prelude::*;
use druid::*;
use qbox_gui::app::view::MainView;
use qbox_gui::app::State;
use qbox_gui::app::{self, get_primary_work_rect};

struct Delegate {
    windows: Vec<WindowId>,
}

impl AppDelegate<State> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> Handled {
        println!("command: {:?}", cmd);
        Handled::No
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut State,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window added, id: {:?}", id);
        self.windows.push(id);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        _data: &mut State,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window removed, id: {:?}", id);
        if let Some(pos) = self.windows.iter().position(|x| *x == id) {
            self.windows.remove(pos);
        }
    }
}

pub fn main() {
    let screen = get_primary_work_rect().unwrap();
    let size = screen.size() * 0.7;
    let position = ((screen.size() - size) / 2.0).to_vec2().to_point();
    #[cfg(target_os = "macos")]
    let size = screen.size();
    #[cfg(target_os = "macos")]
    let position = (((screen.size() - size) / 2.0).to_vec2() - (0.0, 30.0).into()).to_point();

    let app = AppLauncher::with_window(
        WindowDesc::new(MainView::view())
            .title(app::APP_NAME)
            .with_min_size((800., 600.))
            .window_size(size)
            .set_position(position)
            .show_titlebar(false)
            .set_level(WindowLevel::AppWindow)
            .transparent(true),
    );
    // .configure_env(|env, _| {
    //     env.set(theme::WIDGET_PADDING_HORIZONTAL, 5.0);
    //     env.set(theme::WIDGET_PADDING_VERTICAL, 5.0);
    // });

    let state = State::default();

    app.log_to_console().launch(state).expect("launch failed");
}
