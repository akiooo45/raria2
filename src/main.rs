use reqwest::{Url, blocking::Response};
use std::error::Error;

fn get_url(url: &str) -> Result<Response, Box<dyn Error>> {
    let res = reqwest::blocking::get(url)?;
    Ok(res)
}

fn parse_url(url: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(url)?;
    Ok(url)
}

// 新增的函数，使用 ? 运算符和 Result<T, E> 作为返回值
fn fetch_and_print(url_str: &str) -> Result<(), Box<dyn Error>> {
    // 1. 解析 URL。如果失败，函数立即返回 Err。
    let valid_url = parse_url(url_str)?;
    println!("URL format validation successful: {}", valid_url);

    // 2. 获取 URL 内容。如果失败，函数立即返回 Err。
    let res = get_url(valid_url.as_str())?;
    println!("Request successful, Status Code: {}", res.status());

    // 3. 获取响应体文本。如果失败，函数立即返回 Err。
    let body = res.text()?;

    let display_body = body.chars().take(200).collect::<String>();
    println!(
        "Response body content (first 200 chars): {}...",
        display_body
    );

    // 操作全部成功，返回 Ok(())
    Ok(())
}

fn main() {
    let url_str = "https://www.bilibili.com/";

    // 在 main 函数中处理最终的 Result
    if let Err(e) = fetch_and_print(url_str) {
        eprintln!("Operation failed: {}", e);
    }
}
