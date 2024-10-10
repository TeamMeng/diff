use anyhow::{anyhow, Result};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use dialoguer::MultiSelect;
use std::io::stdout;
use std::io::Write;
use xdiff::DiffProfile;
use xdiff::ExtraArgs;
use xdiff::RequestProfile;
use xdiff::ResponseProfile;
use xdiff::{Action, Args, DiffConfig, RunArgs};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(args) => {
            let ret = run(args).await?;
            println!("{}", ret);
        }
        Action::Parse => {
            let ret = parse().await?;
            println!("{}", ret);
        }
    }

    Ok(())
}

async fn parse() -> Result<String> {
    let theme = ColorfulTheme::default();
    let url1: String = Input::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;

    let url2: String = Input::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;

    let name: String = Input::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;

    let res = req1.send(&ExtraArgs::default()).await?;

    let headers = res.get_header_keys();
    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select headers to skip")
        .items(&headers)
        .interact()?;

    let skip_headers = chosen.iter().map(|&i| headers[i].to_string()).collect();

    let res = ResponseProfile::new(skip_headers, vec![]);

    let profile = DiffProfile::new(req1, req2, res);

    let config = DiffConfig::new(vec![(name, profile)].into_iter().collect());

    let result = serde_yaml::to_string(&config)?;

    Ok(result)
}

async fn run(args: RunArgs) -> Result<String> {
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;
    let extra_args = args.extra_params.into();
    let output = profile.diff(extra_args).await?;

    let stdout = stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", output)?;

    Ok(output)
}
