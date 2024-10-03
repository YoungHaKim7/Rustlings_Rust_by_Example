# Test

```bash
$ rustc --test if1.rs

$ ls
if1*    if1.rs  if2.rs  if3.rs


$ ./if1

running 3 tests
test tests::ten_is_bigger_than_eight ... ok
test tests::equal_numbers ... ok
test tests::fortytwo_is_bigger_than_thirtytwo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

  
```

# Run

```bash
$ rustc if1.rs


$ ls
README.md  if1*       if1.rs     if2.rs     if3.rs


$ ./if1
bigger(3, 5) : 5

```
