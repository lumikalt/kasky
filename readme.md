# Käsky

An assembly-esque programming language similar to [SHENZHEN I/O](https://www.zachtronics.com/shenzhen-io/)'s.

## Features

Programs are loops that Halt when there's no more input to consume.
You can run as many instructions per time unit as you want, until the program sleeps. (Note: Due to this, your program must necessarily sleep or it will not halt)

## Documentation

The language has the following arithmetic instructions:

| CMD | inputs | action |
|-----|--------|--------|
| add | R/I [\@R] | adds the input to `acc` and, if set, writes to R |
| sub | R/I [\@R] | subtracts the input from `acc` and, if set, writes to R |
| mul | R/I [\@R] | multiplies the input with `acc` and, if set, writes to R |
| div | R/I [\@R] | divides `acc` by the input and, if set, writes to R |
