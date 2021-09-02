
use crate::config::ConfigStruct;

pub fn default_header(title: &str) -> String
{
	let cfg = crate::config::get_config();
	
	format!("<html><title>{title}</title><body bgcolor=\"{bg_color}\">
			<center><h1>{site_name}</h1></center><hr/><br/><br/>",
			title = title, site_name = cfg.site_name,
			bg_color = cfg.bg_color)
}

pub fn default_footer() -> String
{
	format!("<hr/><center><small>Powered by <a href=\"https://github.com/Subsentient/bleurgh\">Bleurgh</a>, an extremely minimal blog framework.</body></html>")
}
