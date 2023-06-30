use clap::Parser;
use git2::Repository;
use git2::Signature;
use reqwest;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Github username of the account to spoof
    username: String,

    /// Should we trigger a commit append
    #[arg(short, long, default_value_t = true)]
    commit: bool,
}

fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let username = args.username.to_uppercase();
    let username = username.as_str();

    if let Some((username, email)) = get_username_email(username)? {
        eprintln!("Found: {username} {email}");
        if args.commit {
            eprintln!("Applying to git commit amend");
            let repo = match Repository::open(".") {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
            };
            let head = repo.head().unwrap();
            let oid = head.target().unwrap();
            let commit = repo.find_commit(oid).unwrap();
            let sig = Signature::now(username.as_str(), email.as_str()).unwrap();
            commit
                .amend(Some("HEAD"), Some(&sig), Some(&sig), None, None, None)
                .unwrap();
        }
    } else {
        eprint!("Couldn't find users email address");
    }

    Ok(())
}

fn get_username_email(username: &str) -> Result<Option<(String, String)>, reqwest::Error> {
    let repo_url = format!("https://api.github.com/users/{}/repos", username);
    let repo_url = repo_url.as_str();
    let v = get_data(repo_url)?;
    let mut repos: Vec<(&str, &str)> = Vec::new();
    if let Some(items) = v.as_array() {
        for item in items {
            if let (Some(fork), Some(url), Some(push_date)) = (
                item["fork"].as_bool(),
                item["commits_url"].as_str(),
                item["pushed_at"].as_str(),
            ) {
                if !fork {
                    let url = &url[0..url.len() - 6];
                    repos.push((url, push_date));
                }
            }
        }
    }

    // Sort by the push_date
    repos.sort_by(|a, b| b.1.cmp(&a.1));
    for repo in repos {
        eprintln!("Searching: {:?}", repo.0);
        let v = get_data(repo.0)?;

        if let Some(items) = v.as_array() {
            for item in items {
                if let (Some(login), Some(email)) = (
                    item["author"]["login"].as_str(),
                    item["commit"]["author"]["email"].as_str(),
                ) {
                    let original_login = login;
                    let login = login.to_uppercase();
                    let login = login.as_str();

                    if login == username && !email.ends_with("noreply.github.com") {
                        let login = original_login.to_string();
                        let email = email.to_string();
                        return Ok(Some((login, email)));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn get_data(url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("GITHUB-USER-SPOOFER")
        .build()?;
    let res = client.get(url).send()?;
    let body = match res.status() {
        reqwest::StatusCode::OK => res.text()?,
        _ => {
            let b = res.text()?;
            panic!("{:?}", b)
        }
    };

    let v: Value = serde_json::from_str(&body).unwrap();
    Ok(v)
}
