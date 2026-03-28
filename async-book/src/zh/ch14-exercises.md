# 综合练习 🟡

### 练习 1：异步 Echo 服务器

构建一个 TCP echo（回显）服务器，能够并发处理多个客户端连接。

**要求**：
- 监听 `127.0.0.1:8080`
- 接收连接并回显客户端发送的每一行文本
- 能够优雅地处理客户端断开
- 在客户端连接或退出时打印日志

<details>
<summary>🔑 参考答案</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo 服务器已启动，监听端口 :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] 客户端已连接");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("[{addr}] 客户端已断开");
                        break;
                    }
                    Ok(_) => {
                        print!("[{addr}] 回显内容: {line}");
                        if writer.write_all(line.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[{addr}] 读取错误: {e}");
                        break;
                    }
                }
            }
        });
    }
}
```

</details>

---

### 练习 2：带限流的并发抓取器

并发抓取一组 URL，且限制最多同时进行的请求数为 5 个。

<details>
<summary>🔑 参考答案</summary>

```rust
use futures::stream::{self, StreamExt};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5) 会确保流中最多有 5 个 Future 在同时运行
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            async move {
                println!("正在抓取: {url}");
                match reqwest::get(&url).await {
                    Ok(resp) => Ok(resp.text().await.unwrap_or_default()),
                    Err(e) => Err(e.to_string()),
                }
            }
        })
        .buffer_unordered(5) // ← 核心限流逻辑
        .collect()
        .await;

    results
}
```

</details>

---

### 练习 3：带工作池的优雅停机

实现一个任务处理器：
- 拥有基于通道的任务队列
- 启动 N 个 Worker 从队列中领活
- 响应 Ctrl+C：停止领新活，但要干完手里的活再退出

---

### 练习 4：手动实现异步 Mutex

不使用官方提供的 `tokio::sync::Mutex`，利用 `mpsc` 通道手动实现一个简单的异步互斥锁。

*提示*：可以将容量为 1 的通道视为锁的“凭证（Token）”。

---

### 练习 5：流流水线

使用 Stream trait 构建如下流水线：
1. 生成序列 1..=100
2. 过滤掉奇数
3. 计算平方值
4. 限制并发处理度为 10
5. 最后收集所有结果

---

### 练习 6：手动实现 Timeout

不使用 `tokio::select!` 宏，尝试通过手写 `Future` 的方式，实现让一个 Future 与定时器（Timer）赛跑的功能。

***
