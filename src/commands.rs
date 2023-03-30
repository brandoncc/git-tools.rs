use std::process::Command;

use crate::CommandWorkingDirectory;

pub struct CommandConfiguration<'a> {
    cmd: &'a str,
    args: Option<Vec<&'a str>>,
    cwd: CommandWorkingDirectory,
}

#[derive(Debug)]
pub struct SuccessfulCommandExecution {
    pub exit_code: i32,
    pub output: Vec<String>,
}

#[derive(Debug)]
pub struct FailedCommandExecution {
    pub exit_code: i32,
    pub output: Vec<String>,
}

pub type CommandExecutionResult = Result<SuccessfulCommandExecution, FailedCommandExecution>;

fn remove_empty_string_elements(items: Vec<&str>) -> Vec<String> {
    items
        .iter()
        .filter_map(|item: &&str| {
            if item.trim().len() > 1 {
                Some(String::from(*item))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

pub fn run_command(config: CommandConfiguration) -> CommandExecutionResult {
    let mut command = Command::new(config.cmd);

    if let Some(args) = config.args {
        command.args(args);
    }

    command.current_dir(config.cwd);

    let output = command.output();

    if output.is_err() {
        let err = output.unwrap_err();

        panic!(
            "Failed to execute {:?} with error {:?}",
            command.get_program(),
            err.to_string()
        );
    }

    let result =
        output.expect(format!("process {:?} failed to execute", command.get_program(),).as_str());
    let exit_code = result.status.code().unwrap_or(-1);

    let stdout = String::from_utf8(result.stdout).unwrap_or(String::new());
    let items = remove_empty_string_elements(stdout.split('\n').collect::<Vec<&str>>());

    if result.status.success() {
        return Ok(SuccessfulCommandExecution {
            exit_code,
            output: items,
        });
    } else {
        return Err(FailedCommandExecution {
            exit_code,
            output: items,
        });
    }
}

pub fn git_command(args: Vec<&str>, cwd: CommandWorkingDirectory) -> CommandExecutionResult {
    let mut all_args: Vec<&str> = vec!["--no-pager"];
    all_args.extend(args);

    run_command(CommandConfiguration {
        cmd: "git",
        args: Some(all_args),
        cwd,
    })
}
