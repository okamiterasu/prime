use std::path::Path;

use anyhow::Result;

use crate::live;

pub(super) fn load(cache: &Path, manifest: &str) -> Result<String>
{
	let file_path = cache.join(manifest);
	let file = match std::fs::read_to_string(&file_path)
	{
		Ok(contents)=>contents,
		Err(_)=>
		{
			let contents = live::manifest(manifest)?;
			std::fs::write(&file_path, &contents)?;
			contents
		}
	};
	
	// Provided files tend to have erroneous control characters that break
	// parsing and deserialization
	let escaped = file.replace("\r\n", "");
	Ok(escaped)
}