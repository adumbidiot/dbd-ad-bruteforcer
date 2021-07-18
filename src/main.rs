use anyhow::Context;
use std::io::Write;

fn mutate_key(key: &[char], index: usize, list: &mut Vec<Vec<char>>) {
    if index == key.len() {
        list.push(key.to_vec());
    } else {
        mutate_key(key, index + 1, list);

        let mut upper_key = key.to_vec();
        upper_key[index].make_ascii_uppercase();
        mutate_key(&upper_key, index + 1, list);
    }
}

pub fn save_list_to_file(name: &str, list: &[Vec<char>]) -> anyhow::Result<()> {
    let file = std::fs::File::create(name).context("failed to open file")?;
    for entry in list.iter() {
        for c in entry {
            write!(&file, "{}", c)?;
        }

        writeln!(&file)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let base_key = "ytsmtqofpvu";
    let base_key: Vec<_> = base_key.chars().collect();

    let mut key_list = Vec::with_capacity(2usize.pow(base_key.len() as u32));
    mutate_key(&base_key, 0, &mut key_list);

    // save_list_to_file("test.txt", &key_list);

    println!("Generated key list.");

    let client = reqwest::Client::new();
    let (tx, mut rx) = tokio::sync::mpsc::channel(key_list.len());

    for key in key_list {
        let client = client.clone();
        let tx = tx.clone();
        tokio::task::spawn(async move {
            let result = async move {
                let mut url = String::from("https://www.youtube.com/oembed?format=json&url=https://www.youtube.com/watch?v=");
                for c in key.iter() {
                    url.push(*c);
                }
                let response = client.get(url).send().await?;
                let status = response.status();
                let _ = response.text().await.is_ok();
                if status == reqwest::StatusCode::BAD_REQUEST || status == reqwest::StatusCode::NOT_FOUND {
                    Result::<_, anyhow::Error>::Ok((false, key))
                } else {
                    Result::<_, anyhow::Error>::Ok((true, key))
                }
            }.await;
            let _ = tx.send(result).await.is_ok();
        });
    }
    drop(tx);

    while let Some(result) = rx.recv().await {
        match result {
            Ok((is_valid, key)) => {
                if is_valid {
                    println!("Key {} is valid", key.into_iter().collect::<String>());
                }
            }
            Err(e) => {
                eprintln!("Failed to process a key: {:?}", e);
            }
        }
    }
}
