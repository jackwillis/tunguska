#![allow(dead_code)]

use num_rational::Rational32;
use num_traits::{Pow, ToPrimitive, Zero};
use std::fmt::Debug;

const CENTS_PER_OCTAVE: f64 = 1_200.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TuningInterval {
    Cents(f64),
    Ratio(Rational32),
}

impl TuningInterval {
    pub fn new_cents(cents: f64) -> Option<Self> {
        if cents.is_nan() || cents.is_infinite() {
            None
        } else {
            Some(Self::Cents(cents))
        }
    }

    pub fn new_ratio(numer: i32, denom: i32) -> Option<Self> {
        if denom.is_zero() {
            None
        } else {
            let ratio = Rational32::new(numer, denom);
            Some(Self::Ratio(ratio))
        }
    }

    pub fn to_cents(self) -> Self {
        match self {
            Self::Cents(cents) => Self::Cents(cents),
            Self::Ratio(ratio) => {
                let octaves = Self::Ratio(ratio).to_proportion().log2();
                let cents = octaves * CENTS_PER_OCTAVE;
                Self::Cents(cents)
            }
        }
    }

    pub fn to_ratio(self) -> Self {
        match self {
            Self::Cents(cents) => {
                let proportion = Self::Cents(cents).to_proportion();
                let approx_ratio = math::get_approximate_rational(proportion);
                Self::Ratio(approx_ratio)
            }
            Self::Ratio(ratio) => Self::Ratio(ratio),
        }
    }

    pub fn to_proportion(self) -> f64 {
        match self {
            Self::Cents(cents) => 2.0.pow(cents / CENTS_PER_OCTAVE),
            Self::Ratio(ratio) => ratio.to_f64().unwrap(),
        }
    }
}

mod math {
    use num_rational::Rational32;

    pub fn get_approximate_rational(x: f64) -> Rational32 {
        farey(x - x.trunc()) + x.trunc() as i32
    }

    /// Farey algorithm: find closest rational to double in [0, 1]
    fn farey(x: f64) -> Rational32 {
        // See <http://www.johndcook.com/blog/2010/10/20/best-rational-approximation/>.

        const MAX_DENOM: i32 = 200;

        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        let mut d = 1;

        while b < MAX_DENOM && d <= MAX_DENOM {
            let mediant = (a + c) as f64 / (b + d) as f64;

            if almost_equal(x, mediant) {
                if b + d <= MAX_DENOM {
                    return Rational32::new(a + c, b + d);
                } else if d > b {
                    return Rational32::new(c, d);
                } else {
                    return Rational32::new(a, b);
                }
            } else if x > mediant {
                a += c;
                b += d;
            } else {
                c += a;
                d += b;
            }
        }

        if b < MAX_DENOM {
            Rational32::new(c, d)
        } else {
            Rational32::new(a, b)
        }
    }

    fn almost_equal(x: f64, y: f64) -> bool {
        let delta = (x - y).abs();
        delta <= (f64::EPSILON * delta) || delta < f64::MIN_POSITIVE
    }
}
