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
    const CENTS_PER_OCTAVE: i32 = 1200;

    fn ratio_to_cents(ratio: Ratio) -> Cents {
        ratio.to_f64().unwrap().log2() * TuningInterval::CENTS_PER_OCTAVE as Cents
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

    let fifth = TuningInterval::Ratio(Ratio::new(3, 2));
    println!("fifth = {:?}, cents = {}", fifth, fifth.cents());

    let third = TuningInterval::Cents(301.4);
    println!("third = {:?}, cents = {}", third, third.cents());
}
