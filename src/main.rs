use yeelight::*;

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
    Get {
        properties: Vec<yeelight::Property>,
    },
    #[structopt(about = "Toggle light")]
    Toggle,
    #[structopt(about = "Turn on light")]
    On,
    #[structopt(about = "Turn off light")]
    Off,
    #[structopt(about = "Start timer")]
    Timer{ minutes: u64 },
    #[structopt(about = "Clear timer")]
    ClearTimer,
    #[structopt(about = "Set values")]
    Set {
        #[structopt(flatten)]
        property: Prop,
        #[structopt(default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(default_value = "500")]
        duration: u64,
    },
    Flow {
        expression: yeelight::FlowExpresion,
        #[structopt(default_value = "0")]
        count: u8,
        #[structopt(default_value = "Recover")]
        action: yeelight::CfAction,
    }
}

#[derive(Debug, StructOpt)]
enum Prop {
    Power{
        power: yeelight::Power,
        #[structopt(default_value = "Normal")]
        mode: yeelight::Mode
    },
    CT{ color_temperature: u64 },
    RGB{ rgb_value: u32 },
    HSV{ hue: u16,
        #[structopt(default_value = "100")]
        sat: u8
    },
    Bright{ brightness: u8 },
    Name{ name: String },
    Scene{
        class: yeelight::Class,
        val1: u64,
        #[structopt(default_value = "100")]
        val2: u64,
        #[structopt(default_value = "100")]
        val3: u64,
    },
}

fn main() {
    let opt = Options::from_args();
    eprintln!("{:?}", opt);

    let mut bulb = Bulb::connect(&opt.address, opt.port).unwrap();

    let response = match opt.subcommand {
        Command::Toggle => bulb.toggle(),
        Command::On => bulb.set_power(Power::On, Effect::Smooth, 500, Mode::Normal),
        Command::Off => bulb.set_power(Power::Off, Effect::Smooth, 500, Mode::Normal),
        Command::Get{properties} => bulb.get_prop(&Properties(properties)),
        Command::Set{property, effect, duration} => {
            match property {
                Prop::Power{power, mode} => bulb.set_power(power, effect, duration, mode),
                Prop::CT{color_temperature} => bulb.set_ct_abx(color_temperature, effect, duration),
                Prop::RGB{rgb_value} => bulb.set_rgb(rgb_value, effect, duration),
                Prop::HSV{hue, sat} => bulb.set_hsv(hue, sat, effect, duration),
                Prop::Bright{brightness} => bulb.set_bright(brightness, effect, duration),
                Prop::Name{name} => bulb.set_name(&name),
                Prop::Scene{class, val1, val2, val3} => bulb.set_scene(class, val1, val2, val3),
            }
        },
        Command::Timer{minutes} => bulb.cron_add(CronType::Off, minutes),
        Command::ClearTimer => bulb.cron_del(CronType::Off),
        Command::Flow{count, action, expression} => bulb.start_cf(count, action, expression),
    }.unwrap();

    match response {
        Response::Result(result) => result.iter().for_each(|x| println!("{}", x)),
        Response::Error(code, message) => {
            eprintln!("Error (code {}): {}", code, message);
            std::process::exit(code);
        }
    }
}
