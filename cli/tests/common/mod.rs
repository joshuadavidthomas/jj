// Copyright 2020 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod test_environment;

pub use self::test_environment::CommandOutput;
pub use self::test_environment::TestEnvironment;
pub use self::test_environment::TestWorkDir;

pub fn fake_editor_path() -> String {
    let path = assert_cmd::cargo::cargo_bin("fake-editor");
    assert!(path.is_file());
    path.into_os_string().into_string().unwrap()
}

pub fn fake_diff_editor_path() -> String {
    let path = assert_cmd::cargo::cargo_bin("fake-diff-editor");
    assert!(path.is_file());
    path.into_os_string().into_string().unwrap()
}

/// Forcibly enable interactive prompt.
pub fn force_interactive(cmd: &mut assert_cmd::Command) -> &mut assert_cmd::Command {
    cmd.env("JJ_INTERACTIVE", "1")
}

/// Coerces the value type to serialize it as TOML.
pub fn to_toml_value(value: impl Into<toml_edit::Value>) -> toml_edit::Value {
    value.into()
}

/// Returns a string with the last line removed.
///
/// Use this to remove the root error message containing platform-specific
/// content for example.
pub fn strip_last_line(s: &str) -> &str {
    s.trim_end_matches('\n')
        .rsplit_once('\n')
        .map_or(s, |(h, _)| &s[..h.len() + 1])
}

pub fn create_commit(work_dir: &TestWorkDir, name: &str, parents: &[&str]) {
    create_commit_with_files(work_dir, name, parents, &[(name, &format!("{name}\n"))]);
}

pub fn create_commit_with_files(
    work_dir: &TestWorkDir,
    name: &str,
    parents: &[&str],
    files: &[(&str, &str)],
) {
    let parents = match parents {
        [] => &["root()"],
        parents => parents,
    };
    work_dir
        .run_jj_with(|cmd| cmd.args(["new", "-m", name]).args(parents))
        .success();
    for (name, content) in files {
        work_dir.write_file(name, content);
    }
    work_dir
        .run_jj(["bookmark", "create", "-r@", name])
        .success();
}
