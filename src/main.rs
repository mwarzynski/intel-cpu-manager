use intel_pstate::{PState, PStateError};
use std::env;

fn turbo_enable() -> Result<(), PStateError> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(false);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(100)?;
    pstate.set_no_turbo(false)?;
    Ok(())
}

fn hwp_dynamic_boost_enable() -> Result<(), PStateError> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(true);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(100)?;
    pstate.set_no_turbo(false)?;
    Ok(())
}

fn turbo_disable() -> Result<(), PStateError> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(false);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(50)?;
    pstate.set_no_turbo(true)?;
    Ok(())
}

fn print_info() -> Result<(), PStateError> {
    let pstate = PState::new()?;
    let values = pstate.values()?;

    let turbo_enabled = !values.no_turbo;

    println!("turbo: \x1b[33m{}\x1b[0m", turbo_enabled);
    println!(
        "hwp_dynamic_boost: \x1b[33m{:?}\x1b[0m",
        values.hwp_dynamic_boost
    );
    println!(
        "perf_pct_range: \x1b[33m{}-{}\x1b[0m",
        values.min_perf_pct, values.max_perf_pct,
    );

    Ok(())
}

fn main() -> Result<(), PStateError> {
    let args: Vec<String> = env::args().collect();

    let empty_str = String::from("");
    let should_enable_turbo = args.get(1).unwrap_or(&empty_str).eq("true");
    let should_enable_auto = args.get(1).unwrap_or(&empty_str).eq("auto");

    if should_enable_turbo {
        println!("\x1b[31mEnable TURBO!\x1b[0m");
        turbo_enable()?;
    } else if should_enable_auto {
        println!("\x1b[33mAuto selection mode!\x1b[0m");
        hwp_dynamic_boost_enable()?;
    } else if args.len() == 1 {
        print_info()?;
    } else {
        println!("\x1b[32mDisable TURBO.\x1b[0m");
        turbo_disable()?;
    }

    Ok(())
}
