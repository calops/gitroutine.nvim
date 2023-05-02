use std::sync::Arc;

use tokio::sync::RwLock;

use {
    clap::{Args, ColorChoice, Parser, Subcommand},
    gitroutine_macros::CommandArgument,
    nvim_oxi::{
        api::{
            self, notify,
            opts::CreateCommandOpts,
            types::{CommandArgs, CommandComplete, CommandNArgs, LogLevel},
        },
        print, Dictionary, Function, Object,
    },
    serde::Deserialize,
    strum::VariantNames,
};

#[derive(Deserialize, Default, CommandArgument)]
struct Config {}

#[derive(Subcommand)]
enum GithubSubCommand {
    Auth,
}

#[derive(Args)]
struct GithubCommand {
    #[clap(subcommand)]
    command: GithubSubCommand,
}

#[derive(Subcommand, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum GrSubCommand {
    #[clap(name = "github", alias = "gh")]
    Github(GithubCommand),
}

#[derive(Parser, Default)]
#[clap(color = ColorChoice::Always)]
struct GrCommand {
    #[clap(subcommand)]
    command: Option<GrSubCommand>,
}

fn gr(args: CommandArgs) -> Result<(), nvim_oxi::api::Error> {
    let command = match GrCommand::try_parse_from(std::iter::once(":Gr").chain(args.fargs.iter().map(|s| s.as_str()))) {
        Ok(command) => match command {
            GrCommand {
                command:
                    Some(GrSubCommand::Github(GithubCommand {
                        command: GithubSubCommand::Auth,
                    })),
            } => {
                print!("gh auth");
            }
            _ => print!("default"),
        },
        Err(e) => notify(e.to_string().as_str(), LogLevel::Error, &Default::default())?,
    };

    Ok(())
}

fn gr_complete((_lead, _cmd, _cursor_pos): (String, String, usize)) -> nvim_oxi::Result<Vec<String>> {
    //TODO: dynamic completion, preferably with clap directly
    Ok(GrSubCommand::VARIANTS.iter().map(|s| s.to_string()).collect())
}

fn github_auth(_: ()) -> nvim_oxi::Result<()> {
    print!("github auth");
    Ok(())
}

fn setup(config: Config) -> nvim_oxi::Result<()> {
    api::create_user_command(
        "Gr",
        gr,
        &CreateCommandOpts::builder()
            .desc("gitroutine")
            .complete(CommandComplete::CustomList(Function::from_fn(gr_complete)))
            .nargs(CommandNArgs::Any)
            .build(),
    )?;
    Ok(())
}

#[derive(Default)]
struct GlobalState {
    config: Config,
}

#[nvim_oxi::module]
fn gitroutine() -> nvim_oxi::Result<Dictionary> {
    let state = Arc::new(RwLock::new(GlobalState::default()));
    let mut runtime = tokio::runtime::Runtime::new().unwrap(); //TODO: handle errors

    Ok(Dictionary::from_iter([
        ("setup", Object::from(Function::from_fn(setup))),
        (
            "github",
            Object::from(Dictionary::from_iter([(
                "auth",
                Object::from(Function::from_fn(github_auth)),
            )])),
        ),
    ]))
}
