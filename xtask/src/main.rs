use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

const SITE_URL: &str = "https://rust-training.xss.fun";
const SITE_URL_ENV: &str = "RUST_TRAINING_SITE_URL";
const SITE_NAME: &str = "Rust Training Books / Rust 训练书籍";
const SITE_DESCRIPTION: &str =
    "Bilingual Rust training books covering language migration, async, advanced patterns, and engineering practices. / 覆盖语言迁移、异步、进阶模式与工程实践的双语 Rust 培训书籍。";

/// (slug, title, description, category)
/// （slug、标题、描述、分类）
const BOOKS: &[(&str, &str, &str, &str)] = &[
    (
        "c-cpp-book",
        "Rust for C/C++ Programmers / 面向 C/C++ 程序员的 Rust",
        "Move semantics, RAII, FFI, embedded, no_std / 移动语义、RAII、FFI、嵌入式、no_std",
        "bridge",
    ),
    (
        "csharp-book",
        "Rust for C# Programmers / 面向 C# 程序员的 Rust",
        "Best for Swift / C# / Java developers / 适合 Swift / C# / Java 开发者",
        "bridge",
    ),
    (
        "python-book",
        "Rust for Python Programmers / 面向 Python 程序员的 Rust",
        "Dynamic to static typing, GIL-free concurrency / 从动态类型到静态类型，无 GIL 并发",
        "bridge",
    ),
    (
        "async-book",
        "Async Rust: From Futures to Production / Async Rust：从 Future 到生产环境",
        "Tokio, streams, cancellation safety / Tokio、流、取消安全",
        "deep-dive",
    ),
    (
        "rust-patterns-book",
        "Rust Patterns / Rust 模式",
        "Pin, allocators, lock-free structures, unsafe / Pin、分配器、无锁结构、unsafe",
        "advanced",
    ),
    (
        "type-driven-correctness-book",
        "Type-Driven Correctness / 类型驱动的正确性",
        "Type-state, phantom types, capability tokens / 类型状态、幻类型、能力令牌",
        "expert",
    ),
    (
        "engineering-book",
        "Rust Engineering Practices / Rust 工程实践",
        "Build scripts, cross-compilation, coverage, CI/CD / 构建脚本、交叉编译、覆盖率、CI/CD",
        "practices",
    ),
];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live in a workspace subdirectory / xtask 必须位于 workspace 的子目录中")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(),
        Some("serve") => {
            cmd_build();
            cmd_serve();
        }
        Some("deploy") => cmd_deploy(),
        Some("clean") => cmd_clean(),
        Some("--help" | "-h" | "help") | None => print_usage(0),
        Some(other) => {
            eprintln!("Unknown command / 未知命令: {other}\n");
            print_usage(1);
        }
    }
}

fn print_usage(code: i32) {
    let stream: &mut dyn Write = if code == 0 {
        &mut std::io::stdout()
    } else {
        &mut std::io::stderr()
    };
    let _ = writeln!(
        stream,
        "\
Usage / 用法: cargo xtask <COMMAND>

Commands / 命令:
  build    Build all books into site/ (for local preview) / 构建到 site/（本地预览）
  serve    Build and serve at http://localhost:3000 / 构建并在 http://localhost:3000 提供服务
  deploy   Build all books into docs/ (for GitHub Pages) / 构建到 docs/（用于 GitHub Pages）
  clean    Remove site/ and docs/ directories / 删除 site/ 和 docs/ 目录"
    );
    std::process::exit(code);
}

// ── build ────────────────────────────────────────────────────────────

fn cmd_build() {
    build_to("site");
}

fn cmd_deploy() {
    build_to("docs");
    println!(
        "\nGitHub Pages is now deployed through GitHub Actions. \
Local output has been written to docs/. / GitHub Pages 当前通过 GitHub Actions 部署，\
本地输出已写入 docs/。"
    );
}

fn build_to(dir_name: &str) {
    let root = project_root();
    let out = root.join(dir_name);

    if out.exists() {
        fs::remove_dir_all(&out).expect("failed to clean output dir / 清理输出目录失败");
    }
    fs::create_dir_all(&out).expect("failed to create output dir / 创建输出目录失败");

    println!("Building unified site into {dir_name}/ / 正在构建统一站点到 {dir_name}/\n");

    let mut ok = 0u32;
    for &(slug, _, _, _) in BOOKS {
        let book_dir = root.join(slug);
        if !book_dir.is_dir() {
            eprintln!("  ✗ {slug}/ not found, skipping / 未找到，已跳过");
            continue;
        }
        let dest = out.join(slug);
        let status = Command::new("mdbook")
            .args(["build", "--dest-dir"])
            .arg(&dest)
            .current_dir(&book_dir)
            .status()
            .expect("failed to run mdbook - is it installed? / 运行 mdbook 失败，是否已安装？");

        if status.success() {
            rewrite_root_index_links(&dest);
            println!("  ✓ {slug}");
            ok += 1;
        } else {
            eprintln!("  ✗ {slug} FAILED / 构建失败");
        }
    }
    println!("\n  {ok}/{} books built / 已构建", BOOKS.len());

    write_landing_page(&out);
    write_robots_txt(&out);
    write_sitemap(&out);
    println!("\nDone! Output in {dir_name}/ / 完成，输出位于 {dir_name}/");
}

fn category_label(cat: &str) -> &str {
    match cat {
        "bridge" => "Bridge / 桥接",
        "deep-dive" => "Deep Dive / 深入",
        "advanced" => "Advanced / 高级",
        "expert" => "Expert / 专家",
        "practices" => "Practices / 实践",
        _ => cat,
    }
}

fn write_landing_page(site: &Path) {
    let cards: String = BOOKS
        .iter()
        .map(|&(slug, title, desc, cat)| {
            let label = category_label(cat);
            format!(
                r#"    <a class="card cat-{cat}" href="{slug}/">
      <h2>{title} <span class="label">{label}</span></h2>
      <p>{desc}</p>
    </a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{SITE_NAME}</title>
  <meta name="description" content="{SITE_DESCRIPTION}">
  <meta name="robots" content="index,follow,max-image-preview:large">
  <meta name="theme-color" content="#9a3412">
  <link rel="canonical" href="{SITE_URL}/">
  <meta property="og:type" content="website">
  <meta property="og:title" content="{SITE_NAME}">
  <meta property="og:description" content="{SITE_DESCRIPTION}">
  <meta property="og:url" content="{SITE_URL}/">
  <meta property="og:site_name" content="Rust Training">
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="{SITE_NAME}">
  <meta name="twitter:description" content="{SITE_DESCRIPTION}">
  <style>
    :root {{
      --bg: #f5efe4;
      --bg-strong: #efe3cf;
      --card-bg: rgba(255, 251, 245, 0.88);
      --card-border: rgba(91, 58, 20, 0.12);
      --accent: #9a3412;
      --accent-2: #b45309;
      --text: #22170b;
      --muted: #65584b;
      --clr-bridge: #0f9d58;
      --clr-deep-dive: #0284c7;
      --clr-advanced: #c2410c;
      --clr-expert: #7c3aed;
      --clr-practices: #0f766e;
    }}
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      font-family: "Segoe UI Variable Display", "Noto Sans SC", "PingFang SC", "Microsoft YaHei", sans-serif;
      background:
        radial-gradient(circle at top, rgba(180, 83, 9, 0.12), transparent 34%),
        linear-gradient(180deg, #fbf7ef 0%, var(--bg) 42%, var(--bg-strong) 100%);
      color: var(--text);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 2rem 1rem 3rem;
    }}
    .shell {{
      width: min(1120px, 100%);
    }}
    .hero {{
      padding: 2.2rem 2rem 1.4rem;
      margin-bottom: 1.5rem;
      border-radius: 24px;
      background: linear-gradient(135deg, rgba(255,255,255,0.72), rgba(251, 245, 235, 0.92));
      border: 1px solid rgba(154, 52, 18, 0.12);
      box-shadow: 0 18px 48px rgba(91, 58, 20, 0.08);
      backdrop-filter: blur(8px);
    }}
    .eyebrow {{
      display: inline-block;
      margin-bottom: 0.9rem;
      padding: 0.35rem 0.7rem;
      border-radius: 999px;
      background: rgba(180, 83, 9, 0.12);
      color: var(--accent);
      font-size: 0.78rem;
      font-weight: 700;
      letter-spacing: 0.08em;
      text-transform: uppercase;
    }}
    h1 {{
      font-family: Georgia, "Times New Roman", serif;
      font-size: clamp(2.5rem, 5vw, 4.6rem);
      line-height: 0.98;
      margin-bottom: 0.7rem;
      max-width: 12ch;
    }}
    h1 span {{ color: var(--accent); }}
    .subtitle {{
      color: var(--muted);
      font-size: 1.08rem;
      line-height: 1.7;
      max-width: 64ch;
      margin-bottom: 1.2rem;
    }}

    /* Legend */
    .legend {{
      display: flex; flex-wrap: wrap; gap: 0.6rem 1.4rem;
      margin-top: 1.25rem;
      font-size: 0.84rem; color: var(--muted);
    }}
    .legend-item {{ display: flex; align-items: center; gap: 0.35rem; }}
    .legend-dot {{
      width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0;
    }}

    /* Grid & Cards */
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(310px, 1fr));
      gap: 1.5rem;
      width: 100%;
    }}
    .card {{
      background: var(--card-bg);
      border-radius: 18px;
      padding: 1.5rem 1.5rem 1.55rem 1.25rem;
      text-decoration: none;
      color: var(--text);
      transition: transform 0.18s, box-shadow 0.18s, border-color 0.18s;
      border: 1px solid var(--card-border);
      border-left: 5px solid var(--stripe);
      box-shadow: 0 10px 28px rgba(91, 58, 20, 0.06);
    }}
    .card:hover {{
      transform: translateY(-5px);
      box-shadow: 0 16px 34px color-mix(in srgb, var(--stripe) 16%, rgba(91, 58, 20, 0.12));
      border-color: color-mix(in srgb, var(--stripe) 28%, white);
      border-left-color: var(--stripe);
    }}
    .card h2 {{
      font-size: 1.26rem;
      margin-bottom: 0.65rem;
      display: flex;
      align-items: center;
      gap: 0.6rem;
      flex-wrap: wrap;
      line-height: 1.25;
    }}
    .card p  {{ color: var(--muted); font-size: 0.95rem; line-height: 1.6; }}

    /* Category colours */
    .cat-bridge     {{ --stripe: var(--clr-bridge); }}
    .cat-deep-dive  {{ --stripe: var(--clr-deep-dive); }}
    .cat-advanced   {{ --stripe: var(--clr-advanced); }}
    .cat-expert     {{ --stripe: var(--clr-expert); }}
    .cat-practices  {{ --stripe: var(--clr-practices); }}

    /* Label pill */
    .label {{
      font-size: 0.55rem; font-weight: 700; letter-spacing: 0.08em;
      text-transform: uppercase; padding: 0.15em 0.55em;
      border-radius: 4px; white-space: nowrap; flex-shrink: 0;
      color: #fff7ed; background: var(--stripe);
    }}

    footer {{
      margin-top: 2.5rem;
      color: var(--muted);
      font-size: 0.88rem;
      text-align: center;
    }}
    footer a {{ color: var(--accent); }}

    @media (max-width: 700px) {{
      body {{ padding: 1rem 0.9rem 2rem; }}
      .hero {{ padding: 1.4rem 1.1rem 1.15rem; border-radius: 18px; }}
      .grid {{ grid-template-columns: 1fr; gap: 1rem; }}
      .legend {{ gap: 0.55rem 0.9rem; }}
    }}
  </style>
</head>
<body>
  <div class="shell">
    <section class="hero">
      <p class="eyebrow">Rust Curriculum</p>
      <h1>🦀 <span>Rust</span> Training Books / Rust 训练书籍</h1>
      <p class="subtitle">Pick the guide that matches your background, then move from language bridge books into deeper systems, async, and engineering tracks. / 选择与你背景匹配的书籍，再逐步进入更深入的系统、异步与工程实践主题。</p>

      <div class="legend">
        <span class="legend-item"><span class="legend-dot" style="background:var(--clr-bridge)"></span> Bridge &mdash; learn Rust from another language / 桥接：从其他语言迁移到 Rust</span>
        <span class="legend-item"><span class="legend-dot" style="background:var(--clr-deep-dive)"></span> Deep Dive / 深入</span>
        <span class="legend-item"><span class="legend-dot" style="background:var(--clr-advanced)"></span> Advanced / 高级</span>
        <span class="legend-item"><span class="legend-dot" style="background:var(--clr-expert)"></span> Expert / 专家</span>
        <span class="legend-item"><span class="legend-dot" style="background:var(--clr-practices)"></span> Practices / 实践</span>
      </div>
    </section>

    <div class="grid">
{cards}
    </div>
    <footer>Built with <a href="https://rust-lang.github.io/mdBook/">mdBook</a> / 基于 mdBook 构建</footer>
  </div>
</body>
</html>
"##
    );

    let path = site.join("index.html");
    fs::write(&path, html).expect("failed to write index.html / 写入 index.html 失败");
    println!("  ✓ index.html");
}

fn rewrite_root_index_links(book_dir: &Path) {
    let index_path = book_dir.join("index.html");
    let Ok(html) = fs::read_to_string(&index_path) else {
        return;
    };

    let rewritten = html.replace("href=\"ch", "href=\"en/ch");
    if rewritten != html {
        fs::write(&index_path, rewritten)
            .expect("failed to rewrite root index links / 重写根首页链接失败");
    }
}

fn write_robots_txt(site: &Path) {
    let site_url = site_url();
    let content = format!("User-agent: *\nAllow: /\n\nSitemap: {site_url}/sitemap.xml\n");
    fs::write(site.join("robots.txt"), content)
        .expect("failed to write robots.txt / 写入 robots.txt 失败");
    println!("  ✓ robots.txt");
}

fn write_sitemap(site: &Path) {
    let site_url = site_url();
    let mut urls = Vec::new();
    collect_html_urls(site, site, &mut urls, &site_url);
    urls.sort();
    urls.dedup();

    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );
    for url in urls {
        xml.push_str("  <url><loc>");
        xml.push_str(&xml_escape(&url));
        xml.push_str("</loc></url>\n");
    }
    xml.push_str("</urlset>\n");

    fs::write(site.join("sitemap.xml"), xml)
        .expect("failed to write sitemap.xml / 写入 sitemap.xml 失败");
    println!("  ✓ sitemap.xml");
}

fn collect_html_urls(root: &Path, dir: &Path, urls: &mut Vec<String>, site_url: &str) {
    let entries = fs::read_dir(dir).expect("failed to read output directory / 读取输出目录失败");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_html_urls(root, &path, urls, site_url);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("html") {
            continue;
        }
        let file_name = path.file_name().and_then(|name| name.to_str());
        if matches!(file_name, Some("404.html" | "print.html" | "toc.html")) {
            continue;
        }

        let rel = path
            .strip_prefix(root)
            .expect("output path should be under site root / 输出路径必须位于站点根目录下");
        let rel = rel.to_string_lossy().replace('\\', "/");
        let url = if rel == "index.html" {
            format!("{site_url}/")
        } else if let Some(prefix) = rel.strip_suffix("/index.html") {
            format!("{site_url}/{prefix}/")
        } else {
            format!("{site_url}/{rel}")
        };
        urls.push(url);
    }
}

fn site_url() -> String {
    env::var(SITE_URL_ENV)
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| SITE_URL.to_string())
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Resolve `request_target` (HTTP request path, e.g. `/foo/bar?x=1`) to a file under `site_canon`.
/// Returns `None` for traversal attempts, missing files, or paths that escape `site_canon` (symlinks).
/// 将 `request_target`（HTTP 请求路径，如 `/foo/bar?x=1`）解析为 `site_canon` 下的实际文件。
/// 对于目录穿越、文件缺失或通过符号链接逃逸出 `site_canon` 的情况，返回 `None`。
fn resolve_site_file(site_canon: &Path, request_target: &str) -> Option<PathBuf> {
    let path_only = request_target.split('?').next()?.split('#').next()?;
    let decoded = percent_decode_path(path_only);
    if decoded.as_bytes().contains(&0) {
        return None;
    }
    let rel = decoded.trim_start_matches('/');
    let mut file_path = site_canon.to_path_buf();
    if !rel.is_empty() {
        for seg in rel.split('/').filter(|s| !s.is_empty()) {
            if seg == ".." {
                return None;
            }
            file_path.push(seg);
        }
    }
    if file_path.is_dir() {
        file_path.push("index.html");
    }
    let real = fs::canonicalize(&file_path).ok()?;
    if !real.starts_with(site_canon) {
        return None;
    }
    real.is_file().then_some(real)
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn percent_decode_path(input: &str) -> String {
    let mut decoded = Vec::with_capacity(input.len());
    let b = input.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex_val(b[i + 1]), hex_val(b[i + 2])) {
                decoded.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        decoded.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

// ── serve ────────────────────────────────────────────────────────────

fn cmd_serve() {
    let site = project_root().join("site");
    let site_canon = fs::canonicalize(&site).expect(
        "site/ not found - run `cargo xtask build` first (e.g. `cargo xtask serve` runs build automatically) / 未找到 site/，请先运行 `cargo xtask build`（`cargo xtask serve` 会自动先构建）",
    );
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).expect("failed to bind port 3000 / 绑定 3000 端口失败");

    // Handle Ctrl+C gracefully so cargo doesn't report an error.
    // 优雅处理 Ctrl+C，避免 cargo 报错退出。
    ctrlc_exit();

    println!("\nServing at http://{addr} (Ctrl+C to stop) / 正在 http://{addr} 提供服务（按 Ctrl+C 停止）");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);
        let request = String::from_utf8_lossy(&buf[..n]);

        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        if let Some(file_path) = resolve_site_file(&site_canon, path) {
            let body = fs::read(&file_path).unwrap_or_default();
            let mime = guess_mime(&file_path);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&body);
        } else {
            let body = b"404 Not Found";
            let header = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(body);
        }
    }
}

/// Install a Ctrl+C handler that exits cleanly (code 0) instead of
/// letting the OS terminate with STATUS_CONTROL_C_EXIT.
/// 安装 Ctrl+C 处理器，以 0 退出，而不是让操作系统以 STATUS_CONTROL_C_EXIT 终止进程。
fn ctrlc_exit() {
    unsafe {
        libc_set_handler();
    }
}

#[cfg(windows)]
unsafe fn libc_set_handler() {
    // SetConsoleCtrlHandler via the Windows API.
    // 通过 Windows API 调用 SetConsoleCtrlHandler。
    extern "system" {
        fn SetConsoleCtrlHandler(
            handler: Option<unsafe extern "system" fn(u32) -> i32>,
            add: i32,
        ) -> i32;
    }
    unsafe extern "system" fn handler(_ctrl_type: u32) -> i32 {
        std::process::exit(0);
    }
    unsafe {
        SetConsoleCtrlHandler(Some(handler), 1);
    }
}

#[cfg(not(windows))]
unsafe fn libc_set_handler() {
    // On Unix, register SIGINT via libc.
    // 在 Unix 上通过 libc 注册 SIGINT。
    extern "C" {
        fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    }
    extern "C" fn handler(_sig: i32) {
        std::process::exit(0);
    }
    unsafe {
        signal(2 /* SIGINT */, handler);
    }
}

fn guess_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

// ── clean ────────────────────────────────────────────────────────────

fn cmd_clean() {
    let root = project_root();
    for dir_name in ["site", "docs"] {
        let dir = root.join(dir_name);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("failed to remove dir / 删除目录失败");
            println!("Removed {dir_name}/ / 已删除 {dir_name}/");
        }
    }
}
