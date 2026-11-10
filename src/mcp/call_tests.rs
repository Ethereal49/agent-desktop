use super::*;
use serde_json::json;
use std::path::Path;

fn tool_at(path: &[&str]) -> Tool {
    super::super::catalog::tools()
        .into_iter()
        .find(|tool| tool.path == path)
        .expect("tool exists")
}

fn child_args(tool: &Tool, arguments: Value, globals: &[&str]) -> Vec<String> {
    let arguments = arguments.as_object().expect("arguments object");
    let argv = tool.argv(arguments).expect("arguments build argv");
    let globals = globals.iter().map(ToString::to_string).collect::<Vec<_>>();
    child_command(Path::new("agent-desktop"), tool, argv, &globals)
        .get_args()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn ordinary_tools_keep_server_globals_before_their_command() {
    let tool = tool_at(&["snapshot"]);
    let args = child_args(
        &tool,
        json!({ "app": "Finder" }),
        &["--session=abc", "--headed"],
    );
    assert_eq!(
        args,
        ["--session=abc", "--headed", "snapshot", "--app=Finder"]
    );
}

#[test]
fn sandbox_exec_gets_no_server_globals_and_keeps_guest_arguments_in_order() {
    let tool = tool_at(&["sandbox", "exec"]);
    let args = child_args(
        &tool,
        json!({ "name": "dev", "args": ["--headed", "type", "--", "@s8f3k2p9:e1", "--session=guest"] }),
        &["--session=server", "--trace=/tmp/host"],
    );
    assert_eq!(
        args,
        [
            "sandbox",
            "exec",
            "dev",
            "--",
            "--headed",
            "type",
            "--",
            "@s8f3k2p9:e1",
            "--session=guest",
        ]
    );
}
