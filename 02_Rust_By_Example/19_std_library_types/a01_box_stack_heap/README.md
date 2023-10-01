# Result

```
Point occupies 16 bytes on the stack
 f64__mem align of 8
Rectangle occupies 32 bytes on the stack
Boxed point occupies 8 bytes on the stack
Boxed point occupies 8 bytes on the stack
Boxed point occupies 8 bytes on the stack
```


f64 = 8bytes


- Why does an enum require extra memory size? 
https://stackoverflow.com/questions/27324821/why-does-an-enum-require-extra-memory-size

- Tagged union
  - https://en.wikipedia.org/wiki/Tagged_union

# Type layout
https://doc.rust-lang.org/reference/type-layout.html

usize and isize have a size big enough to contain every address on the target platform. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.

Most primitives are generally aligned to their size, although this is platform-specific behavior. In particular, on x86 u64 and f64 are only aligned to 32 bits.