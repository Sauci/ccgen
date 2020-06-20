# ccgen

Crank/cam signal generation

## Requirements

### Hardware
1. ccgen shall rely on a hardware comporting at least the following features:
    1. timer with dual channel output compare feature and interrupt generation on event match

### Speed
1. ccgen shall generate a minimal speed value of 10 rpm.
2. ccgen shall generate a maximal speed value of 20'000 rpm.

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

#### Crank error generation
1. ccgen shall be able to generate a missing tooth on one or several crank teeth. 
2. ccgen shall be able to generate a timing issue (longer or shorter durations than configured ones) on one or several crank teeth. 
3. ccgen shall be able to generate a signal on the missing teeth at request for error generation. 

#### Cam signal generation
1. ccgen shall be able to generate cam signals based on the following configurations:
    1. 6+1
    2. 6+4
2. ccgen shall be able to generate cam signals with inverted polarities.

#### Cam error generation
1. ccgen shall be able to generate a missing tooth on one or several cam teeth. 
2. ccgen shall be able to generate timing issues (longer or short durations than configured) on one or several crank teeth.

