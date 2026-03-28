# 17. Rust 最佳实践 🟢

尽早采用最佳实践将帮助你编写出更地道、更易于维护且更高效的 Rust 代码。本章为从 C/C++ 转向 Rust 的开发人员总结了关键模式和准则。

### 1. 错误处理：`Result` 优于异常 (Exceptions)
Rust 没有异常。相反，对于可能失败的操作，它使用 `Result<T, E>` 类型。使用 `?` 运算符在调用栈中优雅地传播错误。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_content(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?; // 如果打开文件失败，则传播错误
    let mut content = String::new();
    file.read_to_string(&mut content)?; // 如果读取失败，则传播错误
    Ok(content)
}
```

---

### 2. 组合 (Composition) 优于继承 (Inheritance)
Rust 不支持基于类的继承。使用 Trait 定义共享行为，并使用组合来构建复杂类型。

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("正在绘制半径为 {} 的圆", self.radius);
    }
}
```

---

### 3. 迭代器与函数式模式
Rust 的迭代器既强大又高效。尽可能使用它们而不是手动循环，以使你的代码更具表现力且更不易出错。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum();
    println!("偶数平方之和为：{}", sum);
}
```

---

### 4. 文档与测试
Rust 拥有一流的文档和测试支持。对文档注释使用 `///`，并将你的测试放在同一文件或 `tests/` 目录中。

```rust
/// 将两个数字相加。
/// 
/// # 示例
/// 
/// ```
/// let result = my_library::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

---

### 对于 C/C++ 开发者的总结
- **在 C/C++ 中**：最佳实践通常是行业内的“心经”或由外部静态检查工具强制执行的。
- **在 Rust 中**：许多最佳实践直接内置在语言、编译器和标准工具（`cargo fmt`、`cargo clippy`）中。遵守这些约定将使你的代码感觉“地道 (idiomatic)”，并易于其他 Rustacean 阅读和贡献。

***
