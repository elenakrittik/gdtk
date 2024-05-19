use clap::{Args, FromArgMatches, Id};

pub struct Verbosity(pub tracing::Level);

impl Args for Verbosity {
    fn augment_args(_cmd: clap::Command) -> clap::Command {
        todo!()
    }

    fn augment_args_for_update(_cmd: clap::Command) -> clap::Command {
        todo!()
    }

    fn group_id() -> Option<Id> {
        None
    }
}

impl FromArgMatches for Verbosity {
    fn from_arg_matches(_matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        todo!()
    }

    fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        todo!()
    }

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Self::from_arg_matches(matches)
    }

    fn update_from_arg_matches_mut(
        &mut self,
        matches: &mut clap::ArgMatches,
    ) -> Result<(), clap::Error> {
        self.update_from_arg_matches(matches)
    }
}
