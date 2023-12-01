# Käsky

An assembly-esque programming language similar to Shenzhen I/O's.

## Features

Programs are loops that Halt when there's no more input to consume.
You can run as many instructions per time unit as you want, until the program sleeps. (Note: Due to this, your program must necessarily sleep or it will not halt)

## Documentation

The language has the following arithmetic instructions:

| instruction | inputs | description |
| add | R/I [\@R] | adds the input to `acc` or, if set, to R |
| sub | R/I [\@R] | subtracts the input to `acc` or, if set, to R |
