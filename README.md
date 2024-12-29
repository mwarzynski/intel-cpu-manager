# intel-cpu-manager

CLI tool that allows to manage the Intel CPU Turbo Boost and Dynamic Boost.

```
~ intel-cpu-manager
turbo:		true
dynamic_boost:	Some(false)
perf_pct_range:	[10%,100%]
cpu temp:	61°C

cpu0 freq:	0.8GHz
cpu1 freq:	0.8GHz
cpu2 freq:	0.799GHz
cpu3 freq:	0.8GHz
cpu4 freq:	0.8GHz
cpu5 freq:	0.8GHz
cpu6 freq:	0.799GHz
cpu7 freq:	0.8GHz
```

```
$ intel-cpu-manager true
Performance mode.
```

```
$ intel-cpu-manager false
Power_saving mode.
```

```
$ intel-cpu-manager turbo
TURBO!
```
