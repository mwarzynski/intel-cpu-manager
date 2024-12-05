use anyhow::Error;
use intel_pstate::PState;
use sysinfo::{CpuRefreshKind, RefreshKind, System as SysInfoSystem};
use systemstat::{Platform, System};

use std::env;
use std::fs;

fn scaling_governor_read() -> std::io::Result<String> {
    let path = format!("/sys/devices/system/cpu/cpufreq/policy0/scaling_governor");
    let governor = fs::read_to_string(path)?;
    Ok(governor.trim().to_string()) // Trim to remove trailing newline
}

fn scaling_governor_set(governor: &str) -> std::io::Result<()> {
    for policy in 0.. {
        let path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{}/scaling_governor",
            policy
        );
        if fs::metadata(&path).is_err() {
            break; // Stop if the policy path does not exist
        }
        fs::write(path, governor)?;
    }
    Ok(())
}

fn turbo_enable() -> Result<(), Error> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(true);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(100)?;
    pstate.set_no_turbo(false)?;
    scaling_governor_set("performance")?;
    Ok(())
}

fn turbo_disable() -> Result<(), Error> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(false);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(50)?;
    pstate.set_no_turbo(true)?;
    scaling_governor_set("powersave")?;
    Ok(())
}

fn print_info() -> Result<(), Error> {
    let pstate = PState::new()?;
    let values = pstate.values()?;

    let turbo_enabled = !values.no_turbo;

    println!("turbo:\t\t\x1b[33m{}\x1b[0m", turbo_enabled);
    println!(
        "dynamic_boost:\t\x1b[33m{:?}\x1b[0m",
        values.hwp_dynamic_boost
    );
    println!(
        "perf_pct_range:\t\x1b[33m[{}%,{}%]\x1b[0m",
        values.min_perf_pct, values.max_perf_pct,
    );

    let governor = scaling_governor_read()?;
    println!("cpu governor:\t\x1b[33m{}\x1b[0m", governor);

    let sys = System::new();
    let cpu_temp = sys.cpu_temp().unwrap_or_default();
    println!("cpu temp:\t\x1b[33m{}°C\x1b[0m", cpu_temp);

    println!("");

    let s = SysInfoSystem::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );
    for (i, cpu) in s.cpus().iter().enumerate() {
        println!(
            "cpu{} freq:\t\x1b[33m{}GHz\x1b[0m",
            i,
            cpu.frequency() as f32 / 1000.0
        );
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let empty_str = String::from("");
    let should_enable_turbo = args.get(1).unwrap_or(&empty_str).eq("true");

    if should_enable_turbo {
        println!("\x1b[31mEnable TURBO!\x1b[0m");
        turbo_enable()?;
    } else if args.len() == 1 {
        print_info()?;
    } else {
        println!("\x1b[32mDisable TURBO.\x1b[0m");
        turbo_disable()?;
    }

    Ok(())
}
