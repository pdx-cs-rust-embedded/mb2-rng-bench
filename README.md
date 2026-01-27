# mb2-rng: Try out the MB2 RNG
Bart Massey 2026

This code runs the MB2 hardware RNG.

When run, it shows the number of bytes generated in one
second (along with a meaningless sum).

## Features

* `unwhitened`: Disable the whitening circuit for greater
  performance at the expense of quality.

* `blocked`: Generate 1000 bytes at a time to improve
  performance.
