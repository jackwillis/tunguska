use tunguska::{Cents, Ratio, TuningInterval};

fn main() {
    let fifth = Ratio::new(3, 2).unwrap();
    println!("fifth\n-----");
    print_tuning_interval(&fifth);

    let third = Cents::new(301.4).unwrap();
    println!("\nthird\n-----");
    print_tuning_interval(&third);
}

fn print_tuning_interval<T>(interval: &T)
where
    T: TuningInterval,
{
    println!(
        "self     = {:?}\nas_cents = {:?}",
        interval,
        interval.as_cents()
    );
}
