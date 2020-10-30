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

# How to contribute

## Requirements
* **Rust** : 
  * follow intruction for installation [here](https://www.rust-lang.org/tools/install).
  * To install required toolchain : `rustup target add thumbv7m-none-eabi` for cortex-m3 development
  * To install sources for auto-completion : `rustup component add rust-src`
* **Openocd**
* **arm-none-eabi** toolchain: rust depends on arm-non-eabi-gcc in order to link the target executable. A version shall be accessible from the path.
* **arm-none-eabi-gdb**: must also be in the path
* Editing:
  * **VSCode** : install extension *rust-analyzer* + *Cortex-Debug*
  * **Intellij** : install the rust plugin
* **Git**
* Recommanded: serial terminal (ex: [Gatti](https://gitlab.com/susurrus/gattii))

## Compile/run a project

Pull the project, `cd` to its root and type `cargo build`. This command will fetch all the dependencies and then compiled them as well as the project. In order to flash it, two possibilities:
* launch `openocd` (may require administrative rights depending on the user privileges) in a first terminal, and then `cargo run` in a second one to flash and launch GDB.
* Press *F5* in VSCode (only works with *Cortex-Debug* installed and arm-none-eabi-gdb in the path)

## Learning resources/documentation

* [Rust book](https://doc.rust-lang.org/book/)
* [svd2rust documentation (register interface definition + api)](https://docs.rs/svd2rust/0.17.0/svd2rust/)
* [STM32F103 reference manual](http://www.st.com/resource/en/reference_manual/cd00171190.pdf)
* [STM32F103 datasheet](http://www.st.com/content/ccc/resource/technical/document/datasheet/33/d4/6f/1d/df/0b/4c/6d/CD00161566.pdf/files/CD00161566.pdf/jcr:content/translations/en.CD00161566.pdf)
