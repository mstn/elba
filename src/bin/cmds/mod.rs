mod bench;
mod build;
mod check;
mod init;
mod install;
mod lock;
mod new;
mod repl;
mod test;

use clap::{App, ArgMatches};
use elba::util::{config::Config, errors::Res};
use failure::Error;
use slog::{Discard, Logger};

pub type Exec = fn(&mut Config, &ArgMatches) -> Res<()>;

pub fn subcommands() -> Vec<App<'static, 'static>> {
    vec![
        bench::cli(),
        build::cli(),
        check::cli(),
        init::cli(),
        install::cli(),
        new::cli(),
        lock::cli(),
        repl::cli(),
        test::cli(),
    ]
}

pub fn execute_internal(cmd: &str) -> Option<Exec> {
    match cmd {
        "bench" => Some(bench::exec),
        "build" => Some(build::exec),
        "check" => Some(check::exec),
        "init" => Some(init::exec),
        "install" => Some(install::exec),
        "new" => Some(new::exec),
        "lock" => Some(lock::exec),
        "repl" => Some(repl::exec),
        "test" => Some(test::exec),
        _ => None,
    }
}

pub fn execute_external(cmd: &str, args: &ArgMatches) -> Result<(), Error> {
    let ext_args: Vec<&str> = args
        .values_of("")
        .map(|x| x.collect())
        .unwrap_or_else(|| vec![]);
    println!("we're supposed to execute elba-{} {:?}", cmd, ext_args);
    Ok(())
}

pub fn logger(_c: &mut Config) -> Logger {
    Logger::root(Discard, o!())
}
