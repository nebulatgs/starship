use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct RailwayConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub shell_msg: &'a str,
}

/* The trailing double spaces in `symbol` are needed to work around issues with
multiwidth emoji support in some shells. Please do not file a PR to change this
unless you can show that your changes do not affect this workaround.  */
impl<'a> Default for RailwayConfig<'a> {
    fn default() -> Self {
        RailwayConfig {
            format: "on [$symbol$project_name( \\($environment_name\\)) $shell]($style)",
            symbol: "🚅 ",
            style: "bold purple",
            disabled: false,
            shell_msg: "$",
        }
    }
}
