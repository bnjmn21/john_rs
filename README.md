# John.rs

![GitHub License](https://img.shields.io/github/license/bnjmn21/john_rs)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/bnjmn21/john_rs)
[![dependency status](https://deps.rs/repo/github/bnjmn21/john_rs/status.svg)](https://deps.rs/repo/github/bnjmn21/john_rs)
![Website](https://img.shields.io/website?url=https%3A%2F%2Fbnjmn21.github.io%2Fjohn_rs)

## Overview

An emulator for an imaginary basic John von Neumann architecture(s) written in Rust with [egui](https://github.com/emilk/egui/), inspired by the [JOHNNY simulator](https://sourceforge.net/projects/johnnysimulator/).

[*View the web version here!*](bnjmn21.github.io/john_rs)

## Differences to JOHNNY

### Additions

- Customisable RAM size / max integer:
  - **Decimal:**
  - 1,000 / 15,999 (default)
  - 10,000 / 159,999 (6 digits)
  - **Binary:**
  - 4,096 / 65,535 (16 bits)
  - 65,536 / 1,048,575 (20 bits)
  - 1,048,576 / 16,777,216 (might crash your pc)
- Additional instructions in binary mode ([see below](#additional-instructions-in-binary-mode))

### Limitations

- No micro-instructions and therefore no custom instruction sets.
- No control unit simulation.

## Instruction Set

### Default

| Op | Name | Description | Parameter |
|----|------|-------------|-----------|
|`01`|`TAKE`|Load a value from memory into the accumulator|The address to load from|
|`02`|`ADD` |Add a number to the accumulator|The address to take the number from|
|`03`|`SUB` |Subtract a number from the accumulator|The address to take the number from|
|`04`|`SAVE`|Store the value in the accumulator to memory|The address to save to|
|`05`|`JMP` |Jump to a certain address in memory|The address to jump to|
|`06`|`TST` |Jump 2 addresses forwards if the accumulator == 0, otherwise jump 1 as usual| |
|`07`|`INC` |Increment the value in an address by 1|The address whose value is to be incremented|
|`08`|`DEC` |Decrement the value in an address by 1|The address whose value is to be decremented|
|`09`|`NULL`|Set the accumulator and an address to 0|The address whose value shall no longer be positive|
|`10`|`HLT` |Signifies the end of the program||

### Additional Instructions in Binary Mode

| Op | Name | Description | Parameter |
|----|------|-------------|-----------|
|`11`|`AND` |*Bitwise AND* with a value,|whose address is specified here|
|`12`|`OR`  |*Bitwise OR* with a value,|whose address is specified here|
|`13`|`XOR`  |*Bitwise XOR* with a value,|whose address is specified here|
|`14`|`LLS`  |*Logical left shift* with a value,|whose address is specified here|
|`15`|`LRS`  |*Logical right shift* with a value,|whose address is specified here|

### Notes

- *XOR*ing a value with the max int (all ones) is the same as *logical NOT*.
- In binary mode operations such as add and sub are *wrapping* (`1110 + 0010 = 0000`), while in decimal mode they are *saturating* (`15,950 + 100 = 15,999`).
- When executing an undefined opcode (e.g. `00`), the program stops.

## *Self Modifying Code 😨😮*

Many experienced programmers consider *self modifying code 😨😮* to be the single worst thing to have descended to our planet, however for some problems in John.rs / JOHNNY writing self modifying code is necessary.

The reason for this negative connotation is the one big issue with self modifying code:

- It modifies itself

This means your code will probably only run once. Then you can throw you code out the window and start over. Unless you program your code to do so automatically, you will have to painstakingly reset your code after every run. If there just was to tool to do all of this...

### Introducing: John.rs

John.rs automatically saves a copy of your code before you run it, and rewinds to that copy once you reset the cpu.

## File Formats

### JRS (`.jrs.json`)

Stores your settings, emulator state, and everything else.
In the web version your current settings will be automatically saved in `local_storage` so saving is not needed.
(I wouldn't rely on this though since browsers are weird)

### RAM (`.ram`)

Stores just the RAM contents in binary.
*Incompatible with JOHNNY .ram files.*
Thankfully there is `ram2ram.py`.
To convert any old .ram file to the new .ram format,

- Download `ram2ram.py`
- Run `python ram2ram.py <file name of your old .ram file> <file name of your new .ram file>`

`.ram` files can also be compressed using gzip to a `.ram.gz` file.

## Links & Credits

- The [web version](bnjmn21.github.io/john_rs)
- The [JOHNNY simulator](https://sourceforge.net/projects/johnnysimulator/)
- Another [JOHNNY clone](https://dev.inf-schule.de/content/12_rechner/4_johnny/johnny3/) for the web
- The [egui](https://github.com/emilk/egui/) library
