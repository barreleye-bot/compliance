use chrono::{offset::Utc, NaiveDateTime};
use nanoid::nanoid;

pub fn new_unique_id(prefix: &str) -> String {
	format!(
		"{}_{}",
		prefix,
		&nanoid!(
			8,
			&[
				'2', '3', '4', '5', '6', '7', '8', '9', 'a', 'c', 'd', 'e',
				'g', 'h', 'j', 'k', 'm', 'n', 'q', 'r', 's', 't', 'v', 'w',
				'x', 'z',
			]
		),
	)
}

pub fn now() -> NaiveDateTime {
	Utc::now().naive_utc()
}
