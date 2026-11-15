use balanced_ternary::concepts::DigitOperate;
use balanced_ternary::{DataTernary, Ter40, Ternary, Tryte};
use std::fmt::Write;
use std::hint::black_box;

pub trait Runner {
    fn measure<T>(&mut self, name: &str, operation: impl FnMut() -> T);

    fn measure_owned<I, T>(
        &mut self,
        name: &str,
        setup: impl FnMut() -> I,
        operation: impl FnMut(I) -> T,
    );
}

pub fn run(runner: &mut impl Runner) {
    fixed_width(runner);
    native_conversions(runner);

    for width in [16, 40, 128, 512] {
        variable_width(runner, width);
    }
}

fn fixed_width(runner: &mut impl Runner) {
    let left = Tryte::<40>::from_i64(1_234_567_890);
    let right = Tryte::<40>::from_i64(-987_654_321);
    let scalar = Ter40::from_dec(-1_234_567_890);

    runner.measure("tryte40/xor", || black_box(left) ^ black_box(right));

    runner.measure("ter40/digit", || black_box(&scalar).digit(black_box(17)));

    runner.measure("ter40/to_digits", || black_box(&scalar).to_digits());

    let mut buffer = String::with_capacity(40);
    runner.measure("tryte40/display_reused", || {
        buffer.clear();
        write!(&mut buffer, "{}", black_box(&left)).unwrap();
        black_box(buffer.as_str());
    });

    runner.measure("ter40/display_reused", || {
        buffer.clear();
        write!(&mut buffer, "{}", black_box(&scalar)).unwrap();
        black_box(buffer.as_str());
    });
}

fn native_conversions(runner: &mut impl Runner) {
    let value = -1_234_567_890_i64;
    let ternary = Ternary::from_dec(value);
    let word = Tryte::<40>::from_i64(value);
    let stored = DataTernary::from_dec(value);

    runner.measure("native/i64_to_ternary", || {
        Ternary::from_dec(black_box(value))
    });
    runner.measure("native/ternary_to_i64", || {
        black_box(&ternary).checked_to_dec()
    });
    runner.measure("native/i64_to_tryte40", || {
        Tryte::<40>::from_i64(black_box(value))
    });
    runner.measure("native/tryte40_to_i64", || {
        black_box(&word).checked_to_i64()
    });
    runner.measure("native/i64_to_data", || {
        DataTernary::from_dec(black_box(value))
    });
    runner.measure("native/data_to_i64", || black_box(&stored).checked_to_dec());
}

fn variable_width(runner: &mut impl Runner, width: usize) {
    let balanced: String = "+0--+0+".chars().cycle().take(width).collect();
    let ordinary: String = "2101202".chars().cycle().take(width).collect();
    let divisor_text: String = "+-0+".chars().cycle().take(width / 2).collect();

    let left = Ternary::parse(&balanced);
    let right = Ternary::parse(&divisor_text);
    let stored = DataTernary::from_ternary(left.clone());
    let mut buffer = String::with_capacity(width);

    runner.measure(&format!("parse/balanced/{width}"), || {
        Ternary::try_parse(black_box(balanced.as_str())).unwrap()
    });
    runner.measure(&format!("parse/unbalanced/{width}"), || {
        Ternary::try_from_unbalanced(black_box(ordinary.as_str())).unwrap()
    });
    runner.measure(&format!("convert/to_unbalanced/{width}"), || {
        black_box(&left).to_unbalanced()
    });

    runner.measure(&format!("format/to_string/{width}"), || {
        black_box(&left).to_string()
    });
    runner.measure(&format!("format/display_reused/{width}"), || {
        buffer.clear();
        write!(&mut buffer, "{}", black_box(&left)).unwrap();
        black_box(buffer.as_str());
    });

    runner.measure_owned(
        &format!("storage/encode/{width}"),
        || left.clone(),
        |input| DataTernary::from_ternary(black_box(input)),
    );
    runner.measure(&format!("storage/decode_numeric/{width}"), || {
        black_box(&stored).to_ternary()
    });
    runner.measure(&format!("storage/decode_fixed/{width}"), || {
        black_box(&stored).to_fixed_ternary()
    });

    runner.measure(&format!("arithmetic/add/{width}"), || {
        black_box(&left) + black_box(&right)
    });
    runner.measure(&format!("arithmetic/sub/{width}"), || {
        black_box(&left) - black_box(&right)
    });
    runner.measure(&format!("arithmetic/mul/{width}"), || {
        black_box(&left) * black_box(&right)
    });
    runner.measure(&format!("arithmetic/div/{width}"), || {
        black_box(&left) / black_box(&right)
    });
}
