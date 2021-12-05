# Examples

## Success (real size matches expected):

```rust
use pakr_assert_size::*;

#[repr(C, packed)]
#[assert_size(16)]
struct A {
    field1: u64,
    field2: u64,
}

#[assert_size(24)]
#[repr(C, packed)]
struct B {
    field1: u64,
    field2: u64,
    field3: u64,
}
```

## Failure (real size is 24 bytes, expected is 32 bytes):
```rust
use pakr_assert_size::*;

#[assert_size(32)]
#[repr(C, packed)]
struct C {
    field1: u64,
    field2: u64,
    field3: u64,
}
```

# Examples `assert_size_fits`

## Success (real size fits in expected size):

```rust
use pakr_assert_size::*;

// Exact match
#[repr(C, packed)]
#[assert_size_fits(16)]
struct A {
    field1: u64,
    field2: u64,
}

// Fits in match
#[assert_size_fits(32)]
#[repr(C, packed)]
struct B {
    field1: u64,
    field2: u64,
    field3: u64,
}
```

## Failure (real size is 24 bytes, exceeding maximum of 16 bytes):
```rust
use pakr_assert_size::*;

#[assert_size_fits(16)]
#[repr(C, packed)]
struct C {
    field1: u64,
    field2: u64,
    field3: u64,
}
```
