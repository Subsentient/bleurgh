
use crate::common::*;

static CFG_PATH: &str = "/etc/bleurgh.toml";

lazy_static!
{
	static ref CUR_CFG: RwLock<ConfigStruct> =
	{
		RwLock::new(load_config(CFG_PATH).unwrap_or_default())
	};
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigStruct
{
	pub site_name: String,
	pub bg_color: String,
	pub storage_dir: String,
	pub categories: Vec<String>,
}

impl Default for ConfigStruct
{
	fn default() -> ConfigStruct
	{
		ConfigStruct
		{
			site_name: "Unnamed Bleurgh".to_owned(),
			bg_color: "#c3ffff".to_owned(),
			storage_dir: "/var/db/bleurgh".to_owned(),
			categories: Vec::new(),
		}
	}
}

pub fn get_config() -> ConfigStruct
{
	CUR_CFG.read().unwrap().clone()
}

fn load_config(cfg_path: &str) -> Result<ConfigStruct, String>
{
	let cfg_string: String = std::fs::read_to_string(Path::new(cfg_path)).map_err(|e| e.to_string())?;
	
	toml::from_str(&cfg_string).map_err(|e| e.to_string())
}
