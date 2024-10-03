# nocapture test

```bash
$ rustc --test rc1.rs

$ ./rc1 --nocapture

running 1 test
reference count = 1
reference count = 2
Hi from Mercury(Sun)!
reference count = 3
Hi from Venus(Sun)!
reference count = 4
Hi from Earth(Sun)!
reference count = 5
Hi from Mars(Sun)!
reference count = 6
Hi from Jupiter(Sun)!
reference count = 7
Hi from Saturn(Sun)!
reference count = 8
Hi from Uranus(Sun)!
reference count = 9
Hi from Neptune(Sun)!
reference count = 8
reference count = 7
reference count = 6
reference count = 5
reference count = 4
reference count = 3
reference count = 2
reference count = 1
test tests::rc1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s




```

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
