## Benchmarks

edwards_curve_validation                                                                             
                        time:   [3.6296 µs 3.6472 µs 3.6637 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

edwards_add             time:   [10.868 µs 10.921 µs 10.977 µs]                         
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  5 (5.00%) high severe

edwards_subtract        time:   [10.816 µs 10.867 µs 10.916 µs]                              
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  2 (2.00%) high severe

edwards_multiply        time:   [57.441 µs 57.857 µs 58.277 µs]                             
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe

ristretto_curve_validation                                                                             
                        time:   [3.8422 µs 3.8657 µs 3.8892 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

ristretto_add           time:   [12.042 µs 12.118 µs 12.192 µs]                           
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

ristretto_subtract      time:   [11.999 µs 12.086 µs 12.173 µs]                                

ristretto_multiply      time:   [59.107 µs 59.516 µs 59.905 µs]                               
Found 13 outliers among 100 measurements (13.00%)
  9 (9.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

## Compute units assuming 33 ns per CU

edwards curve validation: 111 CU

edwrds add: 331 CU

edwards subtract: 329 CU

edwards multiply: 1753 CU

ristretto curve validation: 117 CU

ristretto add: 367 CU

ristretto subtract: 366 CU

ristretto multiply: 1804 CU
