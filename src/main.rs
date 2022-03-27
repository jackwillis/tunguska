use num_rational::Rational32;
use num_traits::ToPrimitive;

type Cents = f64;
type Ratio = Rational32;

#[derive(Clone, Copy, Debug)]
enum TuningInterval {
    Cents(Cents),
    Ratio(Ratio),
}

impl TuningInterval {
    const CENTS_PER_OCTAVE: Cents = 1_200.0;

    pub fn from_cents(cents: Cents) -> Self {
        Self::Cents(cents)
    }

    pub fn from_ratio(numer: i32, denom: i32) -> Option<Self> {
        if denom == 0 {
            None
        } else {
            Some(Self::Ratio(Ratio::new(numer, denom)))
        }
    }

    fn ratio_to_cents(ratio: Ratio) -> Cents {
        ratio.to_f64().unwrap().log2() * TuningInterval::CENTS_PER_OCTAVE
    }

    pub fn cents(&self) -> Cents {
        match self {
            TuningInterval::Cents(cents) => *cents,
            TuningInterval::Ratio(ratio) => TuningInterval::ratio_to_cents(*ratio),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let fifth = TuningInterval::from_ratio(3, 2).unwrap();
    println!("fifth = {:?}, cents = {}", fifth, fifth.cents());

    let third = TuningInterval::from_cents(301.4);
    println!("third = {:?}, cents = {}", third, third.cents());
}
