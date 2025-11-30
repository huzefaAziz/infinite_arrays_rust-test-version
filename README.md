# InfiniteArrays

A Rust crate for representing arrays with infinite dimension sizes, inspired by [InfiniteArrays.jl](https://github.com/JuliaArrays/InfiniteArrays.jl).

Infinite arrays are by necessity lazy, and support operations like indexing, iteration, and various mathematical operations.

## Features

- **Infinite Ranges**: `OneToInf`, `InfUnitRange`, `InfStepRange`
- **Infinite Arrays**: `Ones`, `Zeros`, and arrays from functions
- **Operations**: `cumsum`, `broadcast`, element-wise operations, scalar operations
- **Caching**: `CachedArray` for mutable infinite arrays

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
infinite-arrays = "0.1.0"
num-traits = "0.2"
num-iter = "0.1"
```

## Examples

### Basic Usage

```rust
use infinite_arrays::*;

// Create an infinite vector of ones
let ones = Ones::new();
assert_eq!(ones.get(0), 1.0);
assert_eq!(ones.get(100), 1.0);

// Create an infinite range
let range = OneToInf::new();
assert_eq!(range.get(0), 1);
assert_eq!(range.get(1), 2);
assert_eq!(range.get(99), 100);
```

### Cumulative Sum

```rust
use infinite_arrays::*;

let ones = Ones::new();
let cumsum_result = cumsum(ones);

assert_eq!(cumsum_result.get(0), 1.0);
assert_eq!(cumsum_result.get(1), 2.0);
assert_eq!(cumsum_result.get(2), 3.0);
assert_eq!(cumsum_result.get(9), 10.0);
```

### Broadcasting

```rust
use infinite_arrays::*;

let ones = Ones::new();
let doubled = broadcast(ones, |x| x * 2.0);

assert_eq!(doubled.get(0), 2.0);
assert_eq!(doubled.get(100), 2.0);
```

### Scalar Operations

```rust
use infinite_arrays::*;

let ones = Ones::new();
let result = add_scalar(ones, 2.0);

assert_eq!(result.get(0), 3.0);
assert_eq!(result.get(100), 3.0);
```

### Cached Arrays (Mutable)

```rust
use infinite_arrays::*;

let ones = Ones::new();
let mut cached = CachedArray::new(ones);

assert_eq!(cached.get(0), 1.0);
cached.set(0, 3.0);
assert_eq!(cached.get(0), 3.0);
assert_eq!(cached.get(1), 1.0);
```

### Infinite Arrays from Functions

```rust
use infinite_arrays::*;

let arr = InfiniteArrayFromFn::new(|i| i * 2);
assert_eq!(arr.get(0), 0);
assert_eq!(arr.get(1), 2);
assert_eq!(arr.get(5), 10);
```

### Iteration

```rust
use infinite_arrays::*;

let ones = Ones::new();
let mut iter = ones.iter();
assert_eq!(iter.next(), Some(1.0));
assert_eq!(iter.next(), Some(1.0));
assert_eq!(iter.next(), Some(1.0));
```

## API Reference

### Ranges

- `OneToInf<T>`: Infinite range starting from 1 (1, 2, 3, ...)
- `InfUnitRange<T>`: Infinite range starting from a given value
- `InfStepRange<T>`: Infinite step range (start, start+step, start+2*step, ...)

### Arrays

- `Ones<T>`: Infinite array filled with ones
- `Zeros<T>`: Infinite array filled with zeros
- `InfiniteArrayFromFn<F, T>`: Infinite array from a function

### Operations

- `cumsum(arr)`: Cumulative sum
- `broadcast(arr, f)`: Apply function to each element
- `add_arrays(a, b)`: Element-wise addition
- `sub_arrays(a, b)`: Element-wise subtraction
- `mul_arrays(a, b)`: Element-wise multiplication
- `div_arrays(a, b)`: Element-wise division
- `add_scalar(arr, scalar)`: Add scalar to each element
- `mul_scalar(arr, scalar)`: Multiply each element by scalar

### Cache

- `CachedArray<T, A>`: Cached infinite array with mutability support

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

