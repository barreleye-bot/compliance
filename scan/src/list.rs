use eyre::Result;
use regex::Regex;

async fn regex_extract(url: &str, regex: &str) -> Result<Vec<String>> {
	let data = reqwest::get(url).await?.text().await?;
	let re = Regex::new(regex).unwrap();

	let addresses: Vec<String> = re
		.captures_iter(&data)
		.filter_map(|cap| cap.get(1).map(|v| v.as_str().to_lowercase()))
		.collect();

	Ok(addresses)
}

pub async fn get_addresses() -> Result<Vec<String>> {
	let mut addresses = vec![];

	let (us_addresses, uk_addresses) =
		tokio::join!(
            regex_extract(
                "https://www.treasury.gov/ofac/downloads/sdn.pip",
                r"Digital Currency Address\s+-\s+\w+\s+([0-9a-zA-Z]+);",
            ),
            regex_extract(
                "https://ofsistorage.blob.core.windows.net/publishlive/2022format/ConList.csv",
                r"Digital Currency Address\s+:\s+\w+\s+([0-9a-zA-Z]+)",
            ),
        );

	addresses.extend(us_addresses?);
	addresses.extend(uk_addresses?);

	addresses.sort_unstable();
	addresses.dedup();

	Ok(addresses)
}
