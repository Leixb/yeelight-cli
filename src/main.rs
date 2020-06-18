use yeelight;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(env)]
    address: String,
    #[structopt(short, long, default_value = "55443")]
    port: u16,
    #[structopt(subcommand)]
    subcommand: Command,
    #[structopt(long)]
    bg: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Get properties")]
    Get { properties: Vec<yeelight::Property> },
    #[structopt(about = "Toggle light")]
    Toggle,
    #[structopt(about = "Turn on light")]
    On {
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
        #[structopt(short, long, default_value = "Normal")]
        mode: yeelight::Mode,
    },
    #[structopt(about = "Turn off light")]
    Off {
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
        #[structopt(short, long, default_value = "Normal")]
        mode: yeelight::Mode,
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
        #[structopt(default_value = "Recover")]
        action: yeelight::CfAction,
    },
    #[structopt(about = "Stop color flow")]
    FlowStop,
    #[structopt(about = "Adjust properties (Bright/CT/Color) (increase/decrease/circle)")]
    Adjust {
        property: yeelight::Prop,
        action: yeelight::AdjustAction,
    },
    #[structopt(about = "Adjust properties (Bright/CT/Color) with perentage (-100~100)")]
    AdjustPercent {
        property: yeelight::Prop,
        percent: i8,
        #[structopt(default_value = "500")]
        duration: u64,
    },
    #[structopt(about = "Connect to music TCP stream")]
    MusicConnect { host: String, port: u32 },
    #[structopt(about = "Stop music mode")]
    MusicStop,
}

#[derive(Debug, StructOpt)]
enum Prop {
    Power {
        power: yeelight::Power,
        #[structopt(default_value = "Normal")]
        mode: yeelight::Mode,
    },
    CT {
        color_temperature: u64,
    },
    RGB {
        rgb_value: u32,
    },
    HSV {
        hue: u16,
        #[structopt(default_value = "100")]
        sat: u8,
    },
    Bright {
        brightness: u8,
    },
    Name {
        name: String,
    },
    Scene {
        class: yeelight::Class,
        val1: u64,
        #[structopt(default_value = "100")]
        val2: u64,
        #[structopt(default_value = "100")]
        val3: u64,
    },
    Default,
}

fn main() {
    let opt = Options::from_args();

    let mut bulb = yeelight::Bulb::connect(&opt.address, opt.port).unwrap();

    let bg = opt.bg;

    let response = match opt.subcommand {
        Command::Toggle => bulb.ch_toggle(bg),
        Command::On {
            effect,
            duration,
            mode,
        } => bulb.ch_set_power(bg, yeelight::Power::On, effect, duration, mode),
        Command::Off {
            effect,
            duration,
            mode,
        } => bulb.ch_set_power(bg, yeelight::Power::Off, effect, duration, mode),
        Command::Get { properties } => bulb.get_prop(&yeelight::Properties(properties)),
        Command::Set {
            property,
            effect,
            duration,
        } => match property {
            Prop::Power { power, mode } => bulb.ch_set_power(bg, power, effect, duration, mode),
            Prop::CT { color_temperature } => {
                bulb.ch_set_ct_abx(bg, color_temperature, effect, duration)
            }
            Prop::RGB { rgb_value } => bulb.ch_set_rgb(bg, rgb_value, effect, duration),
            Prop::HSV { hue, sat } => bulb.ch_set_hsv(bg, hue, sat, effect, duration),
            Prop::Bright { brightness } => bulb.ch_set_bright(bg, brightness, effect, duration),
            Prop::Name { name } => bulb.set_name(&name),
            Prop::Scene {
                class,
                val1,
                val2,
                val3,
            } => bulb.set_scene(class, val1, val2, val3),
            Prop::Default => bulb.ch_set_default(bg),
        },
        Command::Timer { minutes } => bulb.cron_add(yeelight::CronType::Off, minutes),
        Command::TimerClear => bulb.cron_del(yeelight::CronType::Off),
        Command::TimerGet => bulb.cron_get(yeelight::CronType::Off),
        Command::Flow {
            count,
            action,
            expression,
        } => bulb.ch_start_cf(bg, count, action, expression),
        Command::FlowStop => bulb.ch_stop_cf(bg),
        Command::Adjust { action, property } => bulb.ch_set_adjust(bg, action, property),
        Command::AdjustPercent {
            property,
            percent,
            duration,
        } => match property {
            yeelight::Prop::Bright => bulb.ch_adjust_bright(bg, percent, duration),
            yeelight::Prop::Color => bulb.ch_adjust_color(bg, percent, duration),
            yeelight::Prop::CT => bulb.ch_adjust_ct(bg, percent, duration),
        },
        Command::MusicConnect { host, port } => {
            bulb.set_music(yeelight::MusicAction::On, &host, port)
        }
        Command::MusicStop => bulb.set_music(yeelight::MusicAction::Off, "", 0),
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
