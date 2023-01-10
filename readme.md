## Benchmarks

edwards_curve_validation  
 time: [5.2262 µs 5.2348 µs 5.2440 µs]
Found 3 outliers among 100 measurements (3.00%)
3 (3.00%) high mild

edwards_add time: [15.603 µs 15.624 µs 15.646 µs]  
Found 2 outliers among 100 measurements (2.00%)
2 (2.00%) high mild

edwards_subtract time: [15.642 µs 15.659 µs 15.675 µs]  
Found 1 outliers among 100 measurements (1.00%)
1 (1.00%) high mild

edwards_multiply time: [71.780 µs 71.842 µs 71.908 µs]  
Found 5 outliers among 100 measurements (5.00%)
4 (4.00%) high mild
1 (1.00%) high severe

ristretto_curve_validation  
 time: [5.5593 µs 5.5657 µs 5.5739 µs]
Found 6 outliers among 100 measurements (6.00%)
5 (5.00%) high mild
1 (1.00%) high severe

ristretto_add time: [17.172 µs 17.195 µs 17.222 µs]  
Found 5 outliers among 100 measurements (5.00%)
3 (3.00%) high mild
2 (2.00%) high severe

ristretto_subtract time: [17.123 µs 17.136 µs 17.150 µs]  
Found 10 outliers among 100 measurements (10.00%)
4 (4.00%) high mild
6 (6.00%) high severe

ristretto_multiply time: [72.809 µs 72.849 µs 72.893 µs]  
Found 10 outliers among 100 measurements (10.00%)
4 (4.00%) high mild
6 (6.00%) high severe

edwards_msm_1 time: [74.467 µs 74.559 µs 74.656 µs]  
 change: [-1.7119% -1.5556% -1.3971%] (p = 0.00 < 0.05)
Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
4 (4.00%) high mild

edwards_msm_2 time: [100.14 µs 100.25 µs 100.39 µs]  
 change: [-0.1501% -0.0302% +0.0980%] (p = 0.63 > 0.05)
No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
2 (2.00%) high mild
2 (2.00%) high severe

edwards_msm_3 time: [124.63 µs 124.76 µs 124.90 µs]  
 change: [-0.5648% -0.4306% -0.3054%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
5 (5.00%) high mild

edwards_msm_4 time: [150.03 µs 150.19 µs 150.37 µs]  
 change: [+0.1915% +0.3452% +0.4942%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
3 (3.00%) high mild

edwards_msm_5 time: [174.91 µs 175.08 µs 175.26 µs]  
 change: [-0.2752% -0.1041% +0.0713%] (p = 0.25 > 0.05)
No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
3 (3.00%) high mild

ristretto_msm_1 time: [76.207 µs 76.286 µs 76.378 µs]  
 change: [-0.8187% -0.6460% -0.4774%] (p = 0.00 < 0.05)
Change within noise threshold.

ristretto_msm_2 time: [102.76 µs 102.84 µs 102.93 µs]  
 change: [+0.0745% +0.2753% +0.4777%] (p = 0.01 < 0.05)
Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
4 (4.00%) high mild
3 (3.00%) high severe

ristretto_msm_3 time: [128.59 µs 128.76 µs 128.97 µs]  
 change: [+0.5029% +0.6994% +0.8971%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
6 (6.00%) high mild
2 (2.00%) high severe

ristretto_msm_4 time: [154.43 µs 154.55 µs 154.68 µs]  
 change: [+0.3875% +0.5605% +0.7345%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
7 (7.00%) high mild
2 (2.00%) high severe

ristretto_msm_4 #2 time: [179.23 µs 179.45 µs 179.71 µs]  
 change: [+0.1180% +0.2915% +0.4577%] (p = 0.00 < 0.05)
Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
5 (5.00%) high mild
3 (3.00%) high severe

## Compute units assuming 33 ns per CU

edwards curve validation: 159 CU

edwrds add: 473 CU

edwards subtract: 475 CU

edwards multiply: 2177 CU

edwards msm: 1515 + 758 \* n CU

ristretto curve validation: 169 CU

ristretto add: 521 CU

ristretto subtract: 519 CU

ristretto multiply: 2208 CU

ristretto msm: 1515 + 788 \* n CU
