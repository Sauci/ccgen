# ccgen

Crank/cam signal generation

## Requirements

### Hardware
1. ccgen shall rely on a hardware comporting at least the following features:
    1. timer with dual channel output compare feature and interrupt generation on event match

### Speed
1. ccgen shall generate a minimal speed value of 20 rpm.
2. ccgen shall generate a maximal speed value of 12'000 rpm.

### Signals
1. Signal descriptions shall be structured as static configurations. 
2. New configurations shall be easily addable.
3. ccgen shall be able to generate normal and inverted rotation signals, based on a configuration.
4. Configuration choice shall be accessible without restarting ccgen hardware or recompiling ccgen software. 

#### Crank signal generation
1. ccgen shall be able to generate the following crank signals:
    1. 120-2
    2. 120-1
    3. 60-2
    4. 60-1
    5. 30-2
    6. 30-1
2. ccgen shall be able to generate crank signals with inverted polarities. 

#### Cam signal generation
1. ccgen shall be able to generate cam signals based on the following configurations:
    1. 6+1
    2. 6+4
2. ccgen shall be able to generate cam signals with inverted polarities

## Communication protocole

Based on the RTT protocol. See [probe_rtt](https://docs.rs/probe-rs-rtt/0.3.0/probe_rs_rtt/) for host side and 
[rtt_target](https://docs.rs/rtt-target/0.2.0/rtt_target/) for target side implementation.

### Crank types

Config. number | Crank type
:--- | :---
0 | 120-2
1 | 120-1
2 | 60-2
3 | 60-1
4 | 30-2
5 | 30-1
6 | 120-2, inv.
7 | 120-1, inv.
8 | 60-2, inv.
9 | 60-1, inv.
10 | 30-2, inv.
11 | 30-1, inv.

### Cam wheel types

Config. number | Cam type
:--- | :---
0 | 6+4
1 | 6+1
2 | 6+4, inv.
3 | 6+1, inv.



