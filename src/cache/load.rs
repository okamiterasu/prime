use std::path::Path;
use crate::live;

pub(super) fn load(cache: &Path, manifest: &str) -> anyhow::Result<String>
{
	let file_path = cache.join(manifest);
	let file = match std::fs::read_to_string(&file_path)
	{
		Ok(contents)=>contents,
		Err(_)=>
		{
			let contents = live::load_manifest(manifest)?;
			std::fs::write(&file_path, &contents)?;
			contents
		}
	};
	let escaped = file
		.replace(r"\r", "")
		.replace(&['\r', '\n'][..], "");
	Ok(escaped)
}