# Optimizing Java

Doing a little myth busting, refer to book / code [O'Reily "Optimizing Java"](https://github.com/kittylyst/optimizing-java)


Java
```
step 1:  avg time 100738 ns  (0.101 ms)
step 2:  avg time 258189 ns  (0.258 ms) // why does this happen... anything in between turns out to be slower (?)
step 16: avg time 96517 ns   (0.097 ms)
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