use wiringpi::pin::Value::{High, Low};

const W_PINS: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const R_PINS: [u16; 8] = [22, 23, 24, 25, 26, 27, 28, 29];

struct KeyboardHardware {
    w_pins: Vec<wiringpi::pin::OutputPin<wiringpi::pin::WiringPi>>,
    r_pins: Vec<wiringpi::pin::InputPin<wiringpi::pin::WiringPi>>,
}

impl KeyboardHardware {
    pub fn new(pi: &wiringpi::WiringPi<wiringpi::pin::WiringPi>) -> KeyboardHardware {
        KeyboardHardware {
            w_pins: W_PINS
                .iter()
                .map(|p| {
                    let pin = pi.output_pin(*p);
                    pin.digital_write(High);
                    pin
                })
                .collect(),
            r_pins: R_PINS
                .iter()
                .map(|p| {
                    let pin = pi.input_pin(*p);
                    pin.pull_up_dn_control(wiringpi::pin::Pull::Up);
                    pin
                })
                .collect(),
        }
    }

    fn read_pins(&self) -> u8 {
        self.r_pins.iter().fold(0, |acc, x| {
            let p = match x.digital_read() {
                Low => 1,
                High => 0,
            };
            (acc << 1) | p
        })
    }

    pub fn get_current_keys(&self) -> u64 {
        self.w_pins.iter().fold(0, |acc, x| {
            x.digital_write(Low);
            let result = acc << 8 | self.read_pins() as u64;
            x.digital_write(High);
            result
        })
    }
}

#[derive(Debug)]
pub enum Key {
    Code,
    Back,
    Del,
    Tab,
    Return,
    T,
    Down,
    CapsLock,
    ShiftRight,
    ShiftLeft,
    Mod,
    WeirdE,
    Char(char),
    Unknown(u8),
}

impl Key {
    fn from_keycode(code: u8) -> Key {
        match code {
            49 => Key::Code,
            0 => Key::Char('1'),
            3 => Key::Char('2'),
            4 => Key::Char('3'),
            5 => Key::Char('4'),
            1 => Key::Char('5'),
            2 => Key::Char('6'),
            7 => Key::Char('7'),
            6 => Key::Char('8'),
            34 => Key::Char('9'),
            33 => Key::Char('0'),
            42 => Key::Char('ß'),
            41 => Key::Char('`'),
            54 => Key::Back,
            55 => Key::Del,

            50 => Key::Tab,
            8 => Key::Char('q'),
            11 => Key::Char('w'),
            12 => Key::Char('e'),
            13 => Key::Char('r'),
            9 => Key::Char('t'),
            10 => Key::Char('z'),
            15 => Key::Char('u'),
            14 => Key::Char('i'),
            39 => Key::Char('o'),
            37 => Key::Char('p'),
            46 => Key::Char('ü'),
            47 => Key::Char('+'),
            52 => Key::Return,
            45 => Key::T,

            53 => Key::CapsLock,
            16 => Key::Char('a'),
            19 => Key::Char('s'),
            20 => Key::Char('d'),
            21 => Key::Char('f'),
            17 => Key::Char('g'),
            18 => Key::Char('h'),
            23 => Key::Char('j'),
            22 => Key::Char('k'),
            38 => Key::Char('l'),
            36 => Key::Char('ö'),
            44 => Key::Char('ä'),
            43 => Key::Down,

            56 => Key::ShiftLeft,
            24 => Key::Char('y'),
            27 => Key::Char('x'),
            28 => Key::Char('c'),
            29 => Key::Char('v'),
            25 => Key::Char('b'),
            26 => Key::Char('n'),
            31 => Key::Char('m'),
            30 => Key::Char(','),
            35 => Key::Char('.'),
            32 => Key::Char('-'),
            59 => Key::ShiftRight,

            48 => Key::Mod,
            51 => Key::Char(' '),
            40 => Key::WeirdE,

            x => Key::Unknown(x),
        }
    }
}

pub struct KeyboardState {
    hw: KeyboardHardware,
    key_state: u64,
}

impl KeyboardState {
    pub fn new(pi: &wiringpi::WiringPi<wiringpi::pin::WiringPi>) -> KeyboardState {
        let hw = KeyboardHardware::new(pi);
        let key_state = hw.get_current_keys();
        KeyboardState { hw, key_state }
    }
    pub fn update(&mut self) {
        let mut key_state = self.hw.get_current_keys();
        let mut state_diff: u64 = self.key_state ^ key_state;
        self.key_state = key_state;
        let mut index = 0;
        while state_diff != 0 {
            if state_diff & 1 != 0 {
                println!(
                    "{:?} {}",
                    Key::from_keycode(index),
                    if key_state & 1 != 0 {
                        "pressed"
                    } else {
                        "released"
                    }
                );
            }
            state_diff >>= 1;
            key_state >>= 1;
            index += 1;
        }
    }
}
