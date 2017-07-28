extern crate dialoguer;

use std::process::Command;
use dialoguer::Select;

fn run(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd).args(args).output().expect(&format!(
        "failed to execute {}",
        cmd
    ));

    assert!(
        output.status.success(),
        format!("error running {} {:?}", cmd, args)
    );

    String::from_utf8(output.stdout)
        .map(|x| x.trim().to_string())
        .expect("could not convert stdout to utf8")
}

fn main() {
    println!("? Select kubectl context (Use arrow keys)");
    let current_context = run("kubectl", &["config", "current-context"]);
    let contexts_raw = run("kubectl", &["config", "get-contexts", "-o", "name"]);

    let mut contexts = contexts_raw.split("\n").collect::<Vec<&str>>();

    contexts.sort();

    let selected_context_index = Select::new()
        .default(contexts.iter().position(|&x| x == current_context).expect(
            "current selected context should be in selection",
        ))
        .items(&contexts[..])
        .interact()
        .unwrap();

    println!(
        "{}",
        run(
            "kubectl",
            &["config", "use-context", contexts[selected_context_index]],
        )
    );
}
