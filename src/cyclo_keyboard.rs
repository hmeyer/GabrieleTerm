use wiringpi::pin::Value::{High, Low};

const W_PINS: [u16; 3] = [6, 10, 11];
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
                    pin.digital_write(Low);
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
        (0..8).fold(0, |acc, x| {
            for (bit, pin) in self.w_pins.iter().enumerate() {
                pin.digital_write(if (x & (1 << bit)) != 0 { High } else { Low });
            }
            acc << 8 | self.read_pins() as u64
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
            9 => Key::Code,
            56 => Key::Char('1'),
            59 => Key::Char('2'),
            60 => Key::Char('3'),
            61 => Key::Char('4'),
            57 => Key::Char('5'),
            58 => Key::Char('6'),
            63 => Key::Char('7'),
            62 => Key::Char('8'),
            26 => Key::Char('9'),
            25 => Key::Char('0'),
            18 => Key::Char('ß'),
            17 => Key::Char('`'),
            14 => Key::Back,
            15 => Key::Del,

            10 => Key::Tab,
            48 => Key::Char('q'),
            51 => Key::Char('w'),
            52 => Key::Char('e'),
            53 => Key::Char('r'),
            49 => Key::Char('t'),
            50 => Key::Char('z'),
            55 => Key::Char('u'),
            54 => Key::Char('i'),
            31 => Key::Char('o'),
            29 => Key::Char('p'),
            22 => Key::Char('ü'),
            23 => Key::Char('+'),
            12 => Key::Return,
            21 => Key::T,

            13 => Key::CapsLock,
            40 => Key::Char('a'),
            43 => Key::Char('s'),
            44 => Key::Char('d'),
            45 => Key::Char('f'),
            41 => Key::Char('g'),
            42 => Key::Char('h'),
            47 => Key::Char('j'),
            46 => Key::Char('k'),
            30 => Key::Char('l'),
            28 => Key::Char('ö'),
            20 => Key::Char('ä'),
            19 => Key::Down,

            0 => Key::ShiftLeft,
            32 => Key::Char('y'),
            35 => Key::Char('x'),
            36 => Key::Char('c'),
            37 => Key::Char('v'),
            33 => Key::Char('b'),
            34 => Key::Char('n'),
            39 => Key::Char('m'),
            38 => Key::Char(','),
            27 => Key::Char('.'),
            24 => Key::Char('-'),
            3 => Key::ShiftRight,

            8 => Key::Mod,
            11 => Key::Char(' '),
            16 => Key::WeirdE,

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
