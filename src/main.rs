use anyhow::Error;
use clap::Parser;
use intel_pstate::PState;
use std::fs;
use sysinfo::{CpuRefreshKind, RefreshKind, System as SysInfoSystem};
use systemstat::{Platform, System};

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

fn mode_turbo() -> Result<(), Error> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(true);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(100)?;
    pstate.set_no_turbo(false)?;
    scaling_governor_set("performance")?;
    Ok(())
}

fn mode_performance() -> Result<(), Error> {
    let pstate = PState::new()?;
    let _ = pstate.set_hwp_dynamic_boost(true);
    pstate.set_min_perf_pct(0)?;
    pstate.set_max_perf_pct(100)?;
    pstate.set_no_turbo(true)?;
    scaling_governor_set("performance")?;
    Ok(())
}

fn mode_powersave() -> Result<(), Error> {
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

#[derive(Parser)] // requires `derive` feature
#[command(
    name = "intel-cpu-manager",
    about = "Manage Intel CPU power modes",
    version = env!("INTEL_CPU_MANAGER_VERSION")
)]
#[command(bin_name = "intel-cpu-manager")]
#[command(styles = CLAP_STYLING)]
enum CargoCli {
    #[command(about = "Enable power-saving mode for the CPU")]
    PowerSave(PowerSaveArgs),

    #[command(about = "Enable performance mode for the CPU with optional turbo mode")]
    Performance(PerformanceArgs),
}

// See also `clap_cargo::style::CLAP_STYLING`
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(clap::Args)]
#[command(about, long_about = None)]
struct PowerSaveArgs {}

#[derive(clap::Args)]
#[command(about, long_about = None)]
struct PerformanceArgs {
    /// Enable turbo mode.
    #[arg(short, long, default_value_t = false)]
    turbo: bool,
}

fn main() -> Result<(), Error> {
    let args = std::env::args();

    if args.len() <= 1 {
        print_info()?;
        return Ok(());
    }

    let cli = CargoCli::parse();

    match cli {
        CargoCli::PowerSave(_) => {
            mode_powersave()?;
            println!("\x1b[32mPower_saving mode.\x1b[0m");
        }
        CargoCli::Performance(args) => {
            if args.turbo {
                mode_turbo()?;
                println!("\x1b[31mTURBO!\x1b[0m");
            } else {
                mode_performance()?;
                println!("\x1b[33mPerformance mode.\x1b[0m");
            }
        }
    }

    Ok(())
}
