mod utils;

use std::{borrow::Borrow, sync::LazyLock};

use freya::prelude::*;

use discord_modloader::config;
use itertools::Itertools;
use utils::hoverable::hoverable;

pub fn start_gui() {
    let cfg: LaunchConfig<config::Config> = LaunchConfig::new()
        .with_title("Discord Modloader")
        .with_size(1080., 720.)
        .with_decorations(true)
        .with_transparency(true)
        .with_state(config::Config::init());

    launch_cfg(app, cfg);
}

fn get_icon(name: &str) -> Option<Vec<u8>> {
    const LIB_PATH: &'static str = "/usr/lib/discord-modloader";

    let config_path = dirs::config_local_dir().unwrap().join("discord-modloader");
    let icon_path = config_path.join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let lib_path = std::path::PathBuf::from(LIB_PATH);

    let icon_path = lib_path.join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let current_exe = std::env::current_exe().unwrap();
    let icon_path = current_exe.with_file_name("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let cwd = std::env::current_dir().unwrap();
    let icon_path = cwd.join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let icon_path = cwd.join("configs").join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    None
}

fn app() -> Element {
    let ctx = use_context::<config::Config>();

    rsx!(rect {
        direction: "horizontal",
        width: "100%",
        height: "100%",
        background: "rgb(46, 46, 52)",

        ProfileList {}

        // rect {
        //     max_width: "256",
        //     height: "100%",
        //     background: "rgb(46, 46, 52)",
        // }

        rect {
            width: "fill",
            height: "100%",
            background: "rgb(52, 52, 58)",
        }
    })
}

#[component]
fn ProfileList() -> Element {
    let ctx = use_context::<config::Config>();

    let selected: Signal<Option<config::Instance>> = use_signal(|| None);

    rsx!(rect {
        width: "256",
        height: "100%",
        direction: "vertical",
        background: "rgb(46, 46, 52)",
        padding: "8",
        color: "white",
        corner_radius: "8",

        label {
            font_size: "18",
            font_weight: "bold",
            "Profiles"
        }

        ScrollView {
            spacing: "4",

            ProfileListEntry { name: "Test Profile 1", selected: false }

            ProfileListEntry { name: "Test Profile 2", selected: true }

            ProfileListEntry { name: "Test Profile 3", selected: false }

            ProfileListEntry { name: "Test Profile 4", selected: false }
        }

    })
}

#[component]
fn ProfileListEntry(name: String, selected: bool) -> Element {
    let bg_anim = hoverable!(move |ctx| {
        ctx.with(
            AnimColor::new("rgb(52, 52, 58)", "rgb(88, 101, 242)")
                .ease(Ease::InOut)
                .time(100),
        )
    });

    let bg_color = bg_anim.animation.get();

    let bg_color = if selected {
        "rgb(88, 101, 242)"
    } else {
        &bg_color.read().as_string()
    };

    rsx!(rect {
        width: "100%",
        // height: "32",
        padding: "8",
        color: "white",
        corner_radius: "8",

        background: bg_color,
        onmouseenter: bg_anim.onmouseenter,
        onmouseleave: bg_anim.onmouseleave,

        label {
            font_size: "18",
            font_weight: "bold",
            {name}
        }
    })
}
