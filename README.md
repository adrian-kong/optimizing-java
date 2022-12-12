# Optimizing Java

Doing a little myth busting .

Java
```
step 1:  avg time 100738 ns  (0.101 ms)
step 2:  avg time 258189 ns  (0.258 ms)
step 16: avg time 96517 ns   (0.097 ms)
```

Doing the same thing in rust gives:

Rust
```
step 1                   time:   [2.4560 ms 2.4696 ms 2.4844 ms]
                         change: [+0.7431% +1.4771% +2.2076%] (p = 0.00 < 0.05)
                         Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

step 2                   time:   [2.1633 ms 2.1692 ms 2.1754 ms]
                         change: [-0.8617% -0.3127% +0.2004%] (p = 0.26 > 0.05)
                         No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking mem 16: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.5s, enable flat sampling, or reduce sampl
e count to 50.
step 16                  time:   [1.6404 ms 1.6501 ms 1.6603 ms]
                         change: [-1.6976% -0.6922% +0.3429%] (p = 0.20 > 0.05)
                         No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```

Disassembling java using javap

```diff
  public void touchEveryLine();
    Code:
       0: iconst_0
       1: istore_1
       2: iload_1
       3: aload_0
       4: getfield      #14                 // Field testData:[I
       7: arraylength
       8: if_icmpge     27
      11: aload_0
      12: getfield      #14                 // Field testData:[I
      15: iload_1
      16: dup2
      17: iaload
      18: iconst_1
      19: iadd
      20: iastore
+     21: iinc          1, 2    // this is where index counter gets updated
      24: goto          2
      27: return

  public void touchEveryItem();
    Code:
       0: iconst_0
       1: istore_1
       2: iload_1
       3: aload_0
       4: getfield      #14                 // Field testData:[I
       7: arraylength
       8: if_icmpge     27
      11: aload_0
      12: getfield      #14                 // Field testData:[I
      15: iload_1
      16: dup2
      17: iaload
      18: iconst_1
      19: iadd
      20: iastore
+     21: iinc          1, 1
      24: goto          2
      27: return
```

huh whats wrong with iinc