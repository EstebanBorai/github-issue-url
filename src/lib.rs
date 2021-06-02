//! # GitHub Issue URL
//! GitHub prefilled issue URL builder
//!
//! ## Motivation
//!
//! You can have issue form fields prefilled by specifying certain query parameters
//! in the "New Issue" URL (https://github.com/<User | Organization>/<Repository>/issues/new).
//!
//! Example:
//!
//! ```ignore
//! https://github.com/EstebanBorai/github-issue-url/issues/new?
//! title=Null%3A+The+Billion+Dollar+Mistake
//! &body=Null+is+a+flag.+It+represents+different+situations
//! &template=bug_report.md
//! &labels=bug%2Cproduction%2Chigh-severity
//! &assignee=EstebanBorai
//! &milestone=1
//! &projects=1
//! ```
//!
//! This way you can provide a one click "Open Issue" button to your Rust applications,
//! for instance you could have some stack trace, or details read from the host system
//! where your application is running to let the user open an issue on GitHub without
//! the need of specifying system /or application details themselves.
//!
//! ## Contributing
//!
//! Every contribution to this project is welcome! Feel free to open a pull request or an issue.
//!
//! ## License
//!
//! Licensed under both the MIT License and the Apache 2.0 License.
pub mod error;

use url::Url;

use self::error::{Error, Result};

/// GitHub issue struct with support for every field available.
///
/// This struct is holds repository, username or organization name and
/// fields to prefill when opening the issue url.
///
/// # Example
///
/// ```
/// use github_issue_url::Issue;
///
/// const GITHUB_ISSUE_LINK: &str = "https://github.com/EstebanBorai/github-issue-url/issues/new?title=Null%3A+The+Billion+Dollar+Mistake&body=Null+is+a+flag.+It+represents+different+situations+depending+on+the+context+in+which+it+is+used+and+invoked.+This+yields+the+most+serious+error+in+software+development%3A+Coupling+a+hidden+decision+in+the+contract+between+an+object+and+who+uses+it.&template=bug_report.md&labels=bug%2Cproduction%2Chigh-severity&assignee=EstebanBorai&milestone=1&projects=1";
/// const SAMPLE_ISSUE_BODY: &str = r#"Null is a flag. It represents different situations depending on the context in which it is used and invoked. This yields the most serious error in software development: Coupling a hidden decision in the contract between an object and who uses it."#;
///
/// let mut have = Issue::new("github-issue-url", "EstebanBorai").unwrap();
///
/// have.title("Null: The Billion Dollar Mistake");
/// have.body(SAMPLE_ISSUE_BODY);
/// have.template("bug_report.md");
/// have.labels("bug,production,high-severity");
/// have.assignee("EstebanBorai");
/// have.milestone("1");
/// have.projects("1");

/// let have = have.url().unwrap();
///
/// assert_eq!(have, GITHUB_ISSUE_LINK.to_string());
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct Issue<'a> {
    repository_name: &'a str,
    repository_owner: &'a str,
    params: Vec<(&'a str, &'a str)>,
}

/// GitHub Issue including the repository name and the repository owner username.
///
/// Issue fields are kept in a `Vec<(&'a str, &'a str)>` for easy parsing when
/// parsing the URL with query params.
///
/// Every optional param is specified using the setter methods.
impl<'a> Issue<'a> {
    pub fn new(repository_name: &'a str, repository_owner: &'a str) -> Result<Self> {
        if repository_name.is_empty() {
            return Err(Error::EmptyRepositoryName);
        }

        if repository_owner.is_empty() {
            return Err(Error::EmptyRepositoryOwner);
        }

        Ok(Issue {
            repository_name,
            repository_owner,
            params: Vec::new(),
        })
    }

    /// The username of the issue's assignee.
    ///
    /// The issue author requires write access to the repository in order to
    /// use this feature
    pub fn assignee(&mut self, assignee: &'a str) {
        self.params.push(("assignee", assignee));
    }

    /// Prefilled issue body content
    pub fn body(&mut self, body: &'a str) {
        self.params.push(("body", body));
    }

    /// Issue labels separated by comma (`,`).
    /// Example: `bug,production,high-severity`
    ///
    /// The issue author requires write access to the repository in order to
    /// use this feature
    pub fn labels(&mut self, labels: &'a str) {
        self.params.push(("labels", labels));
    }

    /// The ID (number) of the milestone linked to this issue.
    ///
    /// The milestone ID can be found in the Issues/Milestone section.
    ///
    /// https://github.com/<owner>/<repository>/milestone/<milestone id>
    ///
    /// The issue author requires write access to the repository in order to
    /// use this feature
    pub fn milestone(&mut self, milestone: &'a str) {
        self.params.push(("milestone", milestone));
    }

    /// The IDs (number) of the projects to link this issue to separated by
    /// comma (`,`).
    ///
    /// Projects IDs are found in the repository session.
    ///
    /// https://github.com/<owner>/<repository>/projects/<project id>
    ///
    /// The issue author requires write access to the repository in order to
    /// use this feature
    pub fn projects(&mut self, projects: &'a str) {
        self.params.push(("projects", projects));
    }

    /// Prefilled issue title
    pub fn title(&mut self, title: &'a str) {
        self.params.push(("title", title));
    }

    /// The name of the issue template to use when opening the final link.
    /// An issue template lives in .github/ISSUE_TEMPLATE/<issue template name>.md,
    /// if the template you want to use when opening this link is ISSUE_TEMPLATE/bugs.md
    /// the value for `Issue.template` must be `bugs.md`
    pub fn template(&mut self, template: &'a str) {
        self.params.push(("template", template));
    }

    pub fn url(&'a self) -> Result<String> {
        let repository_url = format!(
            "https://github.com/{}/{}/issues/new",
            self.repository_owner, self.repository_name
        );
        let url = Url::parse_with_params(repository_url.as_str(), self.params.iter())
            .map_err(|e| Error::UrlParseError(e.to_string()))?;

        Ok(url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GITHUB_ISSUE_LINK: &str = "https://github.com/EstebanBorai/github-issue-url/issues/new?title=Null%3A+The+Billion+Dollar+Mistake&body=Null+is+a+flag.+It+represents+different+situations+depending+on+the+context+in+which+it+is+used+and+invoked.+This+yields+the+most+serious+error+in+software+development%3A+Coupling+a+hidden+decision+in+the+contract+between+an+object+and+who+uses+it.&template=bug_report.md&labels=bug%2Cproduction%2Chigh-severity&assignee=EstebanBorai&milestone=1&projects=1";
    const SAMPLE_ISSUE_BODY: &str = r#"Null is a flag. It represents different situations depending on the context in which it is used and invoked. This yields the most serious error in software development: Coupling a hidden decision in the contract between an object and who uses it."#;

    #[test]
    fn build_issue_url() {
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
    }

    #[test]
    fn return_error_if_repository_owner_is_invalid() {
        let have = Issue::new("github-issue-url", "");

        assert!(have.is_err());
        assert!(matches!(have, Err(Error::EmptyRepositoryOwner)));
        assert_eq!(
            String::from("Repository owner name is not defined"),
            have.err().unwrap().to_string()
        );
    }

    #[test]
    fn return_error_if_repository_name_is_invalid() {
        let have = Issue::new("", "EstebanBorai");

        assert!(have.is_err());
        assert!(matches!(have, Err(Error::EmptyRepositoryName)));
        assert_eq!(
            String::from("Repository name is not defined"),
            have.err().unwrap().to_string()
        );
    }
}
