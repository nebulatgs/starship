use serde::Deserialize;

use super::{Context, Module, ModuleConfig};

use crate::configs::railway::RailwayConfig;
use crate::formatter::StringFormatter;

/// Creates a module showing if inside a railway shell
///
/// The module will use the `$IN_RAILWAY_SHELL` environment variable to
/// determine if it's inside a railway shell and the name of the project.
///
/// The following options are availables:
///     - `shell_msg` (string)  // change the shell msg
///
/// Will display the following:
///     - project_name (shell)  // $project_name == "project_name" in a railway shell
///     - project_name          // $project_name == "project_name"

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RailwayStarshipOutput {
    name: Option<String>,
    environment_name: Option<String>,
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("railway");
    let config: RailwayConfig = RailwayConfig::try_load(module.config);

    let in_shell = matches!(context.get_env("IN_RAILWAY_SHELL").as_deref(), Some("true"));
    let shell_msg = in_shell.then_some(config.shell_msg.to_string());
    let project = context
        .exec_cmd("railway", &["starship"])
        .iter()
        .flat_map(|output| serde_json::from_str::<RailwayStarshipOutput>(&output.stdout).ok())
        .next()?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "project_name" => project.name.as_ref().map(Ok),
                "environment_name" => project.environment_name.as_ref().map(Ok),
                "shell" => shell_msg.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `railway`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;

    #[test]
    fn no_env_variables() {
        let actual = ModuleRenderer::new("railway").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_env_variables() {
        let actual = ModuleRenderer::new("railway")
            .env("IN_RAILWAY_SHELL", "something_wrong")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}
