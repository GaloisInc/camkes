# Camkes Applications
This repository contains various CAmkES applications, default configurations
and testing scripts.

  apps/       Example applications
  configs/    Default configurations
  test/       Testing scripts

This is not a standalone repository. Checkout [camkes-manifest](https://github.com/GaloisInc/camkes-manifest/blob/rust/README.md) repository for details.

## Rust Applications
Currently we provide two applications:
- `simple` which prints "Hello Rust" on the screen
- `lockserver` which contains one server and three clients and works with seL4 mutex primitive

### Installation
Follow the instructions in [camkes-manifest](https://github.com/GaloisInc/camkes-manifest/blob/rust/README.md).
To build `simple`, use `x86_simple_defconfig`, to build `lockserver`use `x86_lockserver_defconfig`

**Note** that because the current special rust compiler is 64-bit only, the suported targets are x86_64 only (i.e. the build process will produce `capdl-loader-experimental-image-x86_64-pc99` and `kernel-x86_64-pc99` images)


## Other Applications
The applications in this repository are:

- adder: demonstrates the use of dataport wrapper to pass pointers via CAmkES RPC
- epit: demonstrates the use of CAmkES interrupt connector to receive hardware interrupts
- exchangestring: demonstrates how to pass string arguments between components
- filter: demonstrates the use of a component to filter communication
- global-imports:
- hierarchical-attributes:
- hierarchical-components:
- keyboard: demonstrates the use of IO ports and interrupts in a CAmkES component
- lockserver: demonstrates how to use built-in mutex
- multiassembly:
- multiclient: domonstrates multiple clients sharing one connection
- multiplier: demonstrates the use of arrays as arguments
- mutex: demonstrates the use of user-defined connectors in CAmkES
- rotate: demonstrates the use of user-defined types as the arguments of an interface
- swapcounter:
- socket: demonstrates the use of user-defined dataports
- structs: demonstrates the use of struct and array attributes
- terminal: a prototype for a secure terminal system
- uart: demonstrates how to access hardware device memory
