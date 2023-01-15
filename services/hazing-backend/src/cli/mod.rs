use clap::{ValueEnum, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(long, short, value_name = "FILE")]
    pub config: String,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start main service processes
    Run,

    /// Migrate database
    Migrate {
        #[clap(default_value_t = MigrateMode::Up)]
        migrate_mod: MigrateMode,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MigrateMode {
    Up,
    Down
}

impl std::fmt::Display for MigrateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for MigrateMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

impl Cli {
    pub fn command() -> Commands {
        Cli::parse().command
    }

    pub fn config() -> String {
        Cli::parse().config
    }
}