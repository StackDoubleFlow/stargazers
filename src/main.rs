use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use serde_json::Value;
use std::process::Command;

fn do_api(endpoint: &str, per_page: Option<usize>, page: Option<usize>) -> Result<Value> {
    let mut pagination_args = Vec::new();
    if let Some(per_page) = per_page {
        pagination_args.push("-F".to_string());
        pagination_args.push(format!("per_page={}", per_page))
    }
    if let Some(page) = page {
        pagination_args.push("-F".to_string());
        pagination_args.push(format!("page={}", page))
    }
    let mut command = Command::new("gh");
    command
        .args(["api", "--method", "GET", endpoint])
        .args(pagination_args);
    dbg!(&command);
    let output = command.output()?.stdout;
    let res = serde_json::from_slice(&output)?;
    Ok(res)
}

fn main() -> Result<()> {
    let repo = std::env::args()
        .nth(1)
        .context("You must specify a GitHub repository (e.g. MCHPR/MCHPRS)")?;
    let mut stargazers = Vec::new();
    for i in 0.. {
        let res = do_api(
            &format!("/repos/{}/stargazers", repo),
            Some(100),
            Some(i + 1),
        )?;
        let arr = res.as_array().unwrap();
        for obj in arr {
            let login = obj["login"].as_str().unwrap();
            let user = do_api(&format!("/users/{}", login), None, None)?;
            let followers = user["followers"].as_u64().unwrap();
            stargazers.push((followers, login.to_string()))
        }

        println!("Page {} complete.", i);

        if arr.len() < 100 {
            break;
        }
    }

    stargazers.sort_by_key(|s| s.0);
    stargazers.reverse();

    for (followers, name) in stargazers {
        println!("{}: {}", followers, name);
    }

    Ok(())
}
