<div>
  <h1 align="center">GitHub Issue URL</h1>
  <h4 align="center">GitHub prefilled issue URL builder</h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/github-issue-url.svg)](https://crates.io/crates/github-issue-url)
  [![Documentation](https://docs.rs/github-issue-url/badge.svg)](https://docs.rs/github-issue-url)
  ![Build](https://github.com/EstebanBorai/github-issue-url/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/github-issue-url/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/github-issue-url/workflows/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/github-issue-url/workflows/test/badge.svg)

</div>

## Motivation

You can have issue form fields prefilled by specifying certain query parameters
in the "New Issue" URL (https://github.com/<User | Organization>/<Repository>/issues/new).

For example:

```
https://github.com/EstebanBorai/github-issue-url/issues/new?
  title=Null%3A+The+Billion+Dollar+Mistake
  &body=Null+is+a+flag.+It+represents+different+situations
  &template=bug_report.md
  &labels=bug%2Cproduction%2Chigh-severity
  &assignee=EstebanBorai
  &milestone=1
  &projects=1
```

This way you can provide a one click "Open Issue" button to your Rust applications,
for instance you could have some stack trace, or details read from the host system
where your application is running to let the user open an issue on GitHub without
the need of specifying system /or application details themselves.

## Installation

```toml
[dependencies]
github-issue-url = "0.1"
```

## Usage

To create a URL like the one shown above you must use the `Issue` struct included
in this crate, specify the repository owner and the repository name, and then
define the relevant fields such as title, body, template, labels, assginee,
milestone and/or projects, using the "setter-like" methods.

```rust
use github_issue_url::Issue;

const GITHUB_ISSUE_LINK: &str = "https://github.com/EstebanBorai/github-issue-url/issues/new?title=Null%3A+The+Billion+Dollar+Mistake&body=Null+is+a+flag.+It+represents+different+situations+depending+on+the+context+in+which+it+is+used+and+invoked.+This+yields+the+most+serious+error+in+software+development%3A+Coupling+a+hidden+decision+in+the+contract+between+an+object+and+who+uses+it.&template=bug_report.md&labels=bug%2Cproduction%2Chigh-severity&assignee=EstebanBorai&milestone=1&projects=1";

let mut have = Issue::new("github-issue-url", "EstebanBorai").unwrap();

have.title("Null: The Billion Dollar Mistake");
have.body(SAMPLE_ISSUE_BODY);
have.template("bug_report.md");
have.labels("bug,production,high-severity");
have.assignee("EstebanBorai");
have.milestone("1");
have.projects("1");

let have = have.url().unwrap();

assert_eq!(have, GITHUB_ISSUE_LINK.to_string());
```

## Release

```bash
git tag -a v0.1.0 -m "Release Message"
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome! Feel free to open a pull request or an issue.

## License

Licensed under both the MIT License and the Apache 2.0 License.
