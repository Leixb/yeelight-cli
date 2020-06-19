use yeelight;

use structopt::{
    clap::{AppSettings, ArgGroup},
    StructOpt,
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "yeelight-cli",
    about = "A CLI to control your Yeelight smart lights."
)]
#[structopt(global_setting = AppSettings::ColoredHelp)]
struct Options {
    #[structopt(env = "YEELIGHT_ADDR")]
    address: String,
    #[structopt(short, long, default_value = "55443", env = "YEELIGHT_PORT")]
    port: u16,
    #[structopt(subcommand)]
    subcommand: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Get properties")]
    Get {
        #[structopt(possible_values = &yeelight::Property::variants(), case_insensitive = true)]
        #[structopt(required = true)]
        properties: Vec<yeelight::Property>,
    },
    #[structopt(about = "Toggle light")]
    #[structopt(group = ArgGroup::with_name("light"))]
    Toggle {
        #[structopt(long, group = "light")]
        dev: bool,
        #[structopt(long, group = "light")]
        bg: bool,
    },
    #[structopt(about = "Turn on light")]
    On {
        #[structopt(possible_values = &yeelight::Effect::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
        #[structopt(possible_values = &yeelight::Mode::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Normal")]
        mode: yeelight::Mode,
        #[structopt(long)]
        bg: bool,
    },
    #[structopt(about = "Turn off light")]
    Off {
        #[structopt(possible_values = &yeelight::Effect::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
        #[structopt(possible_values = &yeelight::Mode::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Normal")]
        mode: yeelight::Mode,
        #[structopt(long)]
        bg: bool,
    },
    #[structopt(about = "Start timer")]
    Timer { minutes: u64 },
    #[structopt(about = "Clear current timer")]
    TimerClear,
    #[structopt(about = "Get remaining minutes for timer")]
    TimerGet,
    #[structopt(about = "Set values")]
    Set {
        #[structopt(flatten)]
        property: Prop,
        #[structopt(possible_values = &yeelight::Effect::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
    },
    #[structopt(about = "Start color flow")]
    Flow {
        expression: yeelight::FlowExpresion,
        #[structopt(default_value = "0")]
        count: u8,
        #[structopt(possible_values = &yeelight::CfAction::variants(), case_insensitive = true)]
        #[structopt(default_value = "Recover")]
        action: yeelight::CfAction,
        #[structopt(long)]
        bg: bool,
    },
    #[structopt(about = "Stop color flow")]
    FlowStop {
        #[structopt(long)]
        bg: bool,
    },
    #[structopt(about = "Adjust properties (Bright/CT/Color) (increase/decrease/circle)")]
    Adjust {
        #[structopt(possible_values = &yeelight::Prop::variants(), case_insensitive = true)]
        property: yeelight::Prop,
        #[structopt(possible_values = &yeelight::AdjustAction::variants(), case_insensitive = true)]
        action: yeelight::AdjustAction,
        #[structopt(long)]
        bg: bool,
    },
    #[structopt(about = "Adjust properties (Bright/CT/Color) with perentage (-100~100)")]
    #[structopt(setting = AppSettings::AllowNegativeNumbers)]
    AdjustPercent {
        #[structopt(possible_values = &yeelight::Prop::variants(), case_insensitive = true)]
        property: yeelight::Prop,
        percent: i8,
        #[structopt(default_value = "500")]
        duration: u64,
        #[structopt(long)]
        bg: bool,
    },
    #[structopt(about = "Connect to music TCP stream")]
    MusicConnect { host: String, port: u32 },
    #[structopt(about = "Stop music mode")]
    MusicStop,
}

#[derive(Debug, StructOpt)]
enum Prop {
    Power {
        #[structopt(possible_values = &yeelight::Power::variants(), case_insensitive = true)]
        power: yeelight::Power,
        #[structopt(possible_values = &yeelight::Mode::variants(), case_insensitive = true)]
        #[structopt(default_value = "Normal")]
        mode: yeelight::Mode,
        #[structopt(long)]
        bg: bool,
    },
    CT {
        color_temperature: u64,
        #[structopt(long)]
        bg: bool,
    },
    RGB {
        rgb_value: u32,
        #[structopt(long)]
        bg: bool,
    },
    HSV {
        hue: u16,
        #[structopt(default_value = "100")]
        sat: u8,
        #[structopt(long)]
        bg: bool,
    },
    Bright {
        brightness: u8,
        #[structopt(long)]
        bg: bool,
    },
    Name {
        name: String,
    },
    Scene {
        #[structopt(possible_values = &yeelight::Class::variants(), case_insensitive = true)]
        class: yeelight::Class,
        val1: u64,
        #[structopt(default_value = "100")]
        val2: u64,
        #[structopt(default_value = "100")]
        val3: u64,
        #[structopt(long)]
        bg: bool,
    },
    Default {
        #[structopt(long)]
        bg: bool,
    },
}

macro_rules! sel_bg {
    ($obj:tt.$fn:ident ($($p:expr),*) || $fn_bg:ident if $var:tt ) => (
        if $var {
            $obj.$fn_bg($($p),*)
        } else {
            $obj.$fn($($p),*)
        }
    );
}

fn main() {
    let opt = Options::from_args();

    let mut bulb = yeelight::Bulb::connect(&opt.address, opt.port).unwrap();

    let response = match opt.subcommand {
        Command::Toggle{bg, dev} => {
            match (bg, dev) {
                (true, _) => bulb.bg_toggle(),
                (_, true) => bulb.dev_toggle(),
                _ => bulb.toggle(),
            }
        },
        Command::On {
            effect,
            duration,
            mode,
            bg,
        } => sel_bg!(bulb.set_power(yeelight::Power::On, effect, duration, mode) || bg_set_power if bg),
        Command::Off {
            effect,
            duration,
            mode,
            bg,
        } => sel_bg!(bulb.set_power(yeelight::Power::Off, effect, duration, mode) || bg_set_power if bg),
        Command::Get { properties } => bulb.get_prop(&yeelight::Properties(properties)),
        Command::Set {
            property,
            effect,
            duration,
        } => match property {
            Prop::Power { power, mode, bg} => sel_bg!(bulb.set_power(power, effect, duration, mode) || bg_set_power if bg),
            Prop::CT { color_temperature, bg} => sel_bg!(bulb.set_ct_abx(color_temperature, effect, duration) || bg_set_ct_abx if bg),
            Prop::RGB { rgb_value, bg} => sel_bg!(bulb.set_rgb(rgb_value, effect, duration) || bg_set_rgb if bg),
            Prop::HSV { hue, sat, bg} => sel_bg!(bulb.set_hsv(hue, sat, effect, duration) || bg_set_hsv if bg),
            Prop::Bright { brightness, bg} => sel_bg!(bulb.set_bright(brightness, effect, duration) || bg_set_bright if bg),
            Prop::Name { name } => bulb.set_name(yeelight::QuotedString(name)),
            Prop::Scene {
                class,
                val1,
                val2,
                val3,
                bg,
            } => sel_bg!(bulb.set_scene(class, val1, val2, val3) || bg_set_scene if bg),
            Prop::Default{bg} => sel_bg!(bulb.set_default() || bg_set_default if bg),
        },
        Command::Timer { minutes } => bulb.cron_add(yeelight::CronType::Off, minutes),
        Command::TimerClear => bulb.cron_del(yeelight::CronType::Off),
        Command::TimerGet => bulb.cron_get(yeelight::CronType::Off),
        Command::Flow {
            count,
            action,
            expression,
            bg,
        } => sel_bg!(bulb.start_cf(count, action, expression) || bg_start_cf if bg),
        Command::FlowStop { bg } => sel_bg!(bulb.stop_cf() || bg_stop_cf if bg),
        Command::Adjust { action, property, bg } => sel_bg!(bulb.set_adjust(action, property) || bg_set_adjust if bg),
        Command::AdjustPercent {
            property,
            percent,
            duration,
            bg,
        } => match property {
            yeelight::Prop::Bright => sel_bg!(bulb.adjust_bright(percent, duration) || bg_adjust_bright if bg),
            yeelight::Prop::Color => sel_bg!(bulb.adjust_color(percent, duration) || bg_adjust_color if bg),
            yeelight::Prop::CT => sel_bg!(bulb.adjust_ct(percent, duration) || bg_adjust_ct if bg),
        },
        Command::MusicConnect { host, port } => {
            bulb.set_music(yeelight::MusicAction::On, yeelight::QuotedString(host), port)
        }
        Command::MusicStop => bulb.set_music(yeelight::MusicAction::Off, yeelight::QuotedString("".to_string()), 0),
    }
    .unwrap();

    match response {
        yeelight::Response::Result(result) => result.iter().for_each(|x| {
            if x != "ok" {
                println!("{}", x)
            }
        }),
        yeelight::Response::Error(code, message) => {
            eprintln!("Error (code {}): {}", code, message);
            std::process::exit(code);
        }
    }
}
