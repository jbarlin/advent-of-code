Hola!

This is originally based on [Kushagra-0801/adventofcode-rs|https://github.com/Kushagra-0801/adventofcode-rs] but, uh... I've made a large number of tweaks!

## Benchmarks

Thanks Criterion! These are the mean times, more details would be included if you ran them :)

### IntCode

| Day | Original Part 1 | Original Part 2 | After D9 Part 1 | After D9 Part 2 |
|---|---|---|
| 2019 02 | 922.28 ns | 1.9178 ms | 1.6073 us | 3.3455 ms |
| 2019 05 | 1.3979 us | 1.6357 us | 2.0983 us | 2.7135 us |
| 2019 07 | 251.91 us | 362.32 us | 163.17 us | 237.86 us |
| 2019 09 | 5.9426 us | 6.1065 us us | N/A | N/A |

### Non-IntCode

| Day | Part 1 | Part 2|
|---|---|---|
| 2020 22 | 3.2165 us | 339.30 ms |
| 2020 23 | 8.9499 us | 949.69 ms |
| 2020 24 | 607.67 us | 480.86 ms |
| 2020 25 | N/A | 55.118 ms |
| 2019 01 | 2.8544 us | 13.559 us |
| 2019 03 | N/A | 33.630 ms |
| 2019 04 | N/A | 23.571 us |
| 2019 06 | 8.5865 ms | 365.37 us |
| 2019 08 | 518.81 us | 480.96 us |