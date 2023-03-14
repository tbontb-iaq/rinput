# rinput

一个简单的输入库，允许你声明变量的同时读入变量的值

示例：

```rust
use rinput::input;

input!(a: u8, mut c: char, s: String);
println!("{a} {c} {s}");
```

注意：部分行为可能与 C++ 的 `cin` 不一致

## TODO

- 添加输出流

- 使用类似 `stdin()` 的函数代替全局静态变量 `rin`

  - 需要等到 `once_cell` 进入 stable

- 使用泛型特化代替 `read_str` 和 `read_char`

  - 需要泛型特化进入 stable
