use zed_extension_api::{self as zed, Result};

struct JuliaExtension;

impl zed::Extension for JuliaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        zed::set_language_server_installation_status(
            &config.name,
            &zed::LanguageServerInstallationStatus::Cached,
        );


        Ok(zed::Command {
            command: "julia".into(),
            args: vec![
                "--project".into(),
                "-e".into()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(JuliaExtension);
