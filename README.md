# mb2-rng-bench: Evaluate the MB2 RNG
Bart Massey 2026

This code runs the MB2 hardware RNG.

When run, it shows generation statistics. See the code for
details.

## Features

* `biased`: Disable the debiasing circuit to improve
  performance at the expense of quality.

* `blocked`: Generate 1000 bytes at a time to improve
  performance.

* `measure_bias`: Accumulate number of 1 and 0 bits to
  compare.

## Performance

Here's some very rough throughput measurements:

|debiased|blocked|KB/sec|
| ------ | ----- | ---- |
|    1   |   0   |  3.7 |
|    1   |   1   |  8.0 |
|    0   |   0   |  6.1 |
|    0   |   1   |  29  |

With no debiasing and with blocking, typically one second
will produce roughly 115K 1 bits and 116K 0 bits.
