# intel-cpu-manager

CLI tool that allows to manage the Intel CPU Turbo Boost and Dynamic Boost.

```
 ~ intel-cpu-manager
turbo:          false
dynamic_boost:  Some(false)
perf_pct_range: [10%,50%]
cpu governor:   powersave
cpu temp:       59°C

cpu0 freq:      0.4GHz
cpu1 freq:      1.578GHz
cpu2 freq:      1.437GHz
cpu3 freq:      1.711GHz
cpu4 freq:      0.4GHz
cpu5 freq:      1.067GHz
cpu6 freq:      1.677GHz
cpu7 freq:      1.796GHz
```

```
 ~ intel-cpu-manager help
Manage Intel CPU power modes

Usage: intel-cpu-manager <COMMAND>

Commands:
  power-save   Enable power-saving mode for the CPU
  performance  Enable performance mode for the CPU with optional turbo mode
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
 ~ intel-cpu-manager power-save
Power_saving mode.
```

```
 ~ intel-cpu-manager performance
Performance mode.
```

```
 ~ intel-cpu-manager performance --turbo
TURBO!
```
