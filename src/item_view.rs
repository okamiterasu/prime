use crate::relic::Relic;
use crate::structures::{CommonName, UniqueName};

pub trait ItemView
{
	fn common_name(&self) -> CommonName;
	fn unique_name(&self) -> UniqueName;
	fn resurgence_relics(&self) -> &[Relic];
	fn active_relics(&self) -> &[Relic];
	fn available_from_invasion(&self) -> bool;
}