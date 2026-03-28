# 9. 错误处理 🟢

Rust 将错误分为两大类：**可恢复 (Recoverable)** 错误和 **不可恢复 (Unrecoverable)** 错误。

### 1. 不可恢复错误与 `panic!`
在程序无法继续执行的情况下（例如 Bug、资源耗尽），Rust 使用 `panic!` 宏。

```rust
fn main() {
    panic!("崩溃并燃烧");
}
```

当 Panic 发生时，程序会打印一条错误消息，展开 (Unwind) 栈并退出。
常见原因：
- 越界访问数组。
- 对 `None` 或 `Err` 调用 `.unwrap()`。

---

### 2. 使用 `Result<T, E>` 进行可恢复错误处理
大多数错误并不严重到需要程序停止执行。`Result` 枚举代表了成功 (`Ok`) 或失败 (`Err`)。

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("打开文件出错：{error:?}"),
    };
}
```

---

### 3. 使用 `?` 运算符传播错误
你可以使用 `?` 运算符将错误返回给调用者，而不是立即处理它。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // 如果 File::open 失败，它会立即将错误返回给调用者
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

---

### 4. 用于代表“可空 (Nullable)”值的 `Option<T>`
在 Rust 中没有空指针。相反，使用 `Option<T>` 来表示一个可能缺失的值。

```rust
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

fn main() {
    let index = find_word("hello world", "world");
    
    match index {
        Some(i) => println!("找到，索引为 {i}"),
        None => println!("未找到"),
    }
}
```

---

### 5. `unwrap` 与 `expect`
这些方法是快捷方式，它们要么返回内部的值，要么调用 `panic!`。
- **`unwrap()`**：返回内部值，或者以默认消息 Panic。
- **`expect("msg")`**：返回内部值，或者以自定义的消息 Panic（推荐做法）。

```rust
let f = File::open("hello.txt").expect("hello.txt 应该在此项目中");
```

---

### 对 C/C++ 开发者的总结
- **在 C/C++ 中**：你检查返回码 (`if (ret != 0)`) 或针对异常使用 `try/catch`。很容易忘记检查返回码。
- **在 Rust 中**：你 **必须** 处理 `Result` 或 `Option` 类型。如果你忽略了返回的 `Result`，编译器会发出警告。`?` 运算符提供了类似于异常的便捷性，但没有隐藏的控制流。

***
