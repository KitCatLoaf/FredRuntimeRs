# Fred Runtime Rust

Hello! This is the second version of Fred Runtime, now rewritten in Rust. The original can be found at [Fred Runtime](https://github.com/KitCatLoaf/FredRuntime) but it is now deprecated. 

This new version will be written from the ground up in the Rust language.

Just as last time, this is an experimental project meant for fun and not for serious use. I am a beginner and am creating this for fun.

Fred Runtime Rust is now the full replacement as it is feature complete and compatible with all previous fred code. It is now fully rewritten! Enjoy.

## INSTALL
- To install, you will need to run your OS specific installer
  - Linux
    - Navigate into `fredRuntime/installer`
    - run `./linux_install.sh`
    - If needed, run with sudo permission
  - Windows
    - Navigate into `fredRuntime/Output`
    - Run FredSetup.exe
  - MacOS
    - No.

## MANUAL BUILD
  - Install [Rust](https://rustup.rs/)
  - Building
    - To build to path automatically, run `cargo install --path . --force`
    - To build the release version, run `cargo build --release`
    - To compile FredSetup.exe, download [Inno Setup](https://jrsoftware.org/isdl.php)
      - Open `installer.iss` and compile
  
## USAGE
  - To use FredRuntime, simply run fred -h after installing with the above!

FredRuntimeRust is a feature complete alternative to the now deprecate FredRuntime. All code written for the original FredRuntime program should be fully compatible with FredRuntimeRust. 

### FredRuntime v2.1-ALPHA LOG:
- Full, feature complete rewrite into rust of Fred Runtime.
- All libraries including core, fs, io, and time implemented.
- `fred.sleep()` implemented into time library
  - Argument taken in milliseconds
