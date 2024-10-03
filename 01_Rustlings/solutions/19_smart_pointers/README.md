# Result

```bash
$ ./arc1

Sum of offset 1 is 637
Sum of offset 2 is 650
Sum of offset 0 is 624
Sum of offset 4 is 576
Sum of offset 6 is 600
Sum of offset 5 is 588
Sum of offset 3 is 663
Sum of offset 7 is 612


```

- cow

```rs
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
    let my_vec = vec![-1, 0, 1];
    let binding = my_vec.clone();
    let mut input = Cow::from(&binding);
    abs_all(&mut input);
    dbg!(my_vec);
    dbg!(input);
}

```

```bash
$ rustc cow1.rs

$ ./cow1
[cow1.rs:24:5] my_vec = [
    -1,
    0,
    1,
]
[cow1.rs:25:5] input = [
    1,
    0,
    1,
]

```
