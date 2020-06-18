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

    let response = match opt.subcommand {
        Command::Toggle => bulb.toggle(),
        Command::On {
            effect,
            duration,
            mode,
        } => bulb.set_power(yeelight::Power::On, effect, duration, mode),
        Command::Off {
            effect,
            duration,
            mode,
        } => bulb.set_power(yeelight::Power::Off, effect, duration, mode),
        Command::Get { properties } => bulb.get_prop(&yeelight::Properties(properties)),
        Command::Set {
            property,
            effect,
            duration,
        } => match property {
            Prop::Power { power, mode } => bulb.set_power(power, effect, duration, mode),
            Prop::CT { color_temperature } => bulb.set_ct_abx(color_temperature, effect, duration),
            Prop::RGB { rgb_value } => bulb.set_rgb(rgb_value, effect, duration),
            Prop::HSV { hue, sat } => bulb.set_hsv(hue, sat, effect, duration),
            Prop::Bright { brightness } => bulb.set_bright(brightness, effect, duration),
            Prop::Name { name } => bulb.set_name(&name),
            Prop::Scene {
                class,
                val1,
                val2,
                val3,
            } => bulb.set_scene(class, val1, val2, val3),
            Prop::Default => bulb.set_default(),
        },
        Command::Timer { minutes } => bulb.cron_add(yeelight::CronType::Off, minutes),
        Command::TimerClear => bulb.cron_del(yeelight::CronType::Off),
        Command::TimerGet => bulb.cron_get(yeelight::CronType::Off),
        Command::Flow {
            count,
            action,
            expression,
        } => bulb.start_cf(count, action, expression),
        Command::FlowStop => bulb.stop_cf(),
        Command::Adjust { action, property } => bulb.set_adjust(action, property),
        Command::AdjustPercent {
            property,
            percent,
            duration,
        } => match property {
            yeelight::Prop::Bright => bulb.adjust_bright(percent, duration),
            yeelight::Prop::Color => bulb.adjust_color(percent, duration),
            yeelight::Prop::CT => bulb.adjust_ct(percent, duration),
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
