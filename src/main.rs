type CentsInner = f64;
type RatioInner = num_rational::Ratio<i32>;

trait TuningInterval {
    const CENTS_PER_OCTAVE: CentsInner = 1_200.0;
    fn cents(&self) -> CentsInner;
    fn ratio(&self) -> RatioInner;
}

#[derive(Clone, Copy, Debug)]
struct Cents(CentsInner);

impl Cents {
    fn new(cents: CentsInner) -> Option<Self> {
        Some(Cents(cents))
    }
}

impl TuningInterval for Cents {
    fn cents(&self) -> CentsInner {
        self.0
    }

    fn ratio(&self) -> RatioInner {
        //FIXME
        RatioInner::new(1, 1)
    }
}

#[derive(Clone, Copy, Debug)]
struct Ratio(RatioInner);

impl Ratio {
    fn new(numer: i32, denom: i32) -> Option<Self> {
        Some(Ratio(RatioInner::new(numer, denom)))
    }
}

impl TuningInterval for Ratio {
    fn cents(&self) -> CentsInner {
        let ratio_as_double = num_traits::ToPrimitive::to_f64(&self.0).unwrap();
        let octaves = ratio_as_double.log2();
        octaves * Self::CENTS_PER_OCTAVE
    }

    fn ratio(&self) -> RatioInner {
        self.0
    }
}

fn main() {
    println!("Hello, world!");

    let fifth = Ratio::new(3, 2).unwrap();
    println!("fifth = {:?}, cents = {}", fifth, fifth.cents());

    let third = Cents::new(301.4).unwrap();
    println!("third = {:?}, cents = {}", third, third.cents());
}
