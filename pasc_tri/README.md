# pasc_tri

## This is the Pascal's triangle builder

### Pascal's triangle rules:
1. The *nth* row and the *kth* column is denoted as (n, k);
2. Position (0, 0) is equal to 1;
3. Position (n, k) is obtained by the following equation *(n, k) = (n-1, k-1) + (n-1, k)*, for
   any non-negative 0 <= *k* <= *n*.

### Example of Pascal's triangle for n = 5 and k = 5:
```rust
1
1 1
1 2 1
1 3 3 1
1 4 6 4 1
```

### Usage:
`./pasc_tri n_row`
