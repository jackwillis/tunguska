use num_rational::Rational32;

const CENTS_PER_OCTAVE: f64 = 1_200.0;

trait TuningInterval: PartialEq {
    fn as_cents(&self) -> Cents;
    fn as_ratio(&self) -> Ratio;

    // fn add_cents(&self, cents: Cents) -> Box<dyn TuningInterval>;
    // fn mul_ratio(&self, ratio: Ratio) -> Box<dyn TuningInterval>;

    // fn add_octaves(&self, octaves: i32) -> Box<dyn TuningInterval> {
    //     const CENTS_PER_OCTAVE: i32 = 1200;
    //     self.add_cents(Cents((octaves * CENTS_PER_OCTAVE) as f64))
    // }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cents(f64);

impl Cents {
    fn new(cents: f64) -> Self {
        Cents(cents)
    }
}

impl TuningInterval for Cents {
    fn as_cents(&self) -> Cents {
        *self
    }

    fn as_ratio(&self) -> Ratio {
        //FIXME
        Ratio::new(1, 1).unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Ratio(Rational32);

impl Ratio {
    fn new(numer: i32, denom: i32) -> Option<Self> {
        if denom == 0 {
            None
        } else {
            Some(Self(Rational32::new(numer, denom)))
        }
    }
}

impl TuningInterval for Ratio {
    fn as_cents(&self) -> Cents {
        let ratio_as_double = num_traits::ToPrimitive::to_f64(&self.0).unwrap();
        let octaves = ratio_as_double.log2();
        Cents::new(octaves * CENTS_PER_OCTAVE)
    }

    fn as_ratio(&self) -> Ratio {
        *self
    }
}

fn main() {
    println!("Hello, world!");

    let fifth = Ratio::new(3, 2).unwrap();
    println!("fifth = {:?}, cents = {:?}", fifth, fifth.as_cents());

    let third = Cents::new(301.4);
    println!("third = {:?}, cents = {:?}", third, third.as_cents());
}

#[cfg(test)]
mod tests {
    use crate::{Cents, TuningInterval};

    #[test]
    fn cents_as_cents_is_identity() {
        let xs = vec![
            0.0,
            -0.0,
            0.7,
            1.0,
            3.14,
            f64::INFINITY,
            -f64::INFINITY,
            f64::MIN,
            f64::MAX - f64::MIN,
            f64::MIN_POSITIVE,
        ];

        for x in xs {
            let c = Cents::new(x);
            assert_eq!(c, c.as_cents());
        }
    }
}
