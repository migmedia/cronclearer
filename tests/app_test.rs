use similar_asserts::assert_eq;
use std::{
    env,
    process::{Command, Output},
};

fn exec(args: &[&str]) -> Output {
    let prog_file = env::current_exe()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path.join("cronclearer")
        })
        .expect("Executable should have been found.");
    let mut prog = Command::new(prog_file);
    prog.args(args);
    prog.output().unwrap()
}

fn to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn just_stderr() {
    let output = exec(&["./tests/just_stderr.sh", "-x", "-V"]);
    assert_eq!(
        to_string(&output.stdout),
        r#"# Failure or error output for the command:
`./tests/just_stderr.sh -x -V`

## Exit-code: 0

## Err output:
```
Text in std-err
```

## Std output:
```
Text in std-out

```
"#
    );
    assert!(output.status.code().unwrap() == 0);
}

#[test]
fn just_stderr_ignore_text() {
    let output = exec(&["-i", "./tests/just_stderr.sh"]);
    assert_eq!(to_string(&output.stdout), "");
    assert!(output.status.code().unwrap() == 0);
}

#[test]
fn return_one() {
    let output = exec(&["./tests/return_one.sh"]);
    assert_eq!(
        to_string(&output.stdout),
        r#"# Failure or error output for the command:
`./tests/return_one.sh `

## Exit-code: 1

## Err output:
```

```

## Std output:
```
Text in std-out

```

## Trace output:
```
+ echo 'Text in std-out'
+ exit 1

```
"#
    );
    assert!(output.status.code().unwrap() == 1);
}

#[test]
fn return_two_ignore_text() {
    let output = exec(&["-i", "./tests/return_one.sh"]);
    assert_eq!(
        to_string(&output.stdout),
        r#"# Failure or error output for the command:
`./tests/return_one.sh `

## Exit-code: 1

## Err output:
```

```

## Std output:
```
Text in std-out

```

## Trace output:
```
+ echo 'Text in std-out'
+ exit 1

```
"#
    );
    assert!(output.status.code().unwrap() == 1);
}
