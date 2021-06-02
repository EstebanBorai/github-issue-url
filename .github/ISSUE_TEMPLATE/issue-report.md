---
name: Issue Report
about: Report an existent issue in our latest version
title: "[Issue Report]"
labels: issue
assignees: ''
---

**Description**

Describe what happened

**Steps to reproduce**

> This is an example on how to define the steps to reproduce section

1. Add a `use` statement at the top of your module and import the `IssueBuilder`
from the `github_issue_url` crate.

```rust
use github_issue_url::IssueBuilder;
```

2. Create an issue using the `IssueBuilder` as follows:

```rust
let builder = IssueBuilder::new("Octocat", "StickerPack")
  .build();
```

**Expected Behavior**

Describe what you expect instead, or whats wrong
