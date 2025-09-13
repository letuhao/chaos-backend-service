---
name: CI Failure Report
about: Report a Continuous Integration failure
title: 'CI Failure: [Brief Description]'
labels: ['bug', 'ci', 'needs-investigation']
assignees: []

---

## CI Failure Report

### Basic Information
- **Workflow**: <!-- e.g., Continuous Integration, Nightly Builds, Release -->
- **Job**: <!-- e.g., Feature Matrix, Quality, Performance -->
- **Run ID**: <!-- GitHub Actions run ID -->
- **Branch**: <!-- Branch where failure occurred -->
- **Commit SHA**: <!-- Full commit hash -->
- **Date**: <!-- Date of failure -->

### Failure Details
<!-- Provide the error message and relevant logs -->

```
[Paste error message/logs here]
```

### Feature Configuration
<!-- If this is a feature matrix failure, specify which features were being tested -->
- **Features**: <!-- e.g., default, moka-cache, heavy-deps, all-features -->
- **Platform**: <!-- e.g., ubuntu-latest, windows-latest, macos-latest -->
- **Rust Version**: <!-- e.g., stable, nightly, 1.89 -->

### Steps to Reproduce
<!-- How can this failure be reproduced locally? -->

1. 
2. 
3. 

### Expected Behavior
<!-- What should have happened instead? -->

### Environment Information
<!-- Fill out what's relevant -->
- **OS**: <!-- e.g., Ubuntu 22.04, Windows Server 2022, macOS 13 -->
- **Rust Version**: <!-- Output of `rustc --version` -->
- **Cargo Version**: <!-- Output of `cargo --version` -->
- **Dependencies**: <!-- Any relevant dependency versions -->

### Investigation Notes
<!-- What have you already tried or discovered? -->

- [ ] Checked if this is a known issue
- [ ] Verified the failure is reproducible
- [ ] Checked recent changes that might have caused this
- [ ] Reviewed related PRs or commits

### Potential Causes
<!-- List possible reasons for the failure -->

- [ ] Code change introduced a bug
- [ ] Dependency update broke compatibility
- [ ] Feature flag configuration issue
- [ ] Platform-specific issue
- [ ] Flaky test
- [ ] Infrastructure/CI runner issue
- [ ] Performance regression
- [ ] Security vulnerability detected

### Priority Assessment
<!-- How critical is this failure? -->

- [ ] **Critical** - Blocks all development (main branch broken)
- [ ] **High** - Blocks releases or major features
- [ ] **Medium** - Affects specific features or platforms
- [ ] **Low** - Minor issue or flaky test

### Additional Context
<!-- Any other information that might be helpful -->

### Related Issues/PRs
<!-- Link to related issues or pull requests -->

- Related to #
- Caused by #
- Blocks #

### Checklist for Resolution
<!-- Items to verify when fixing this issue -->

- [ ] Root cause identified
- [ ] Fix implemented and tested locally
- [ ] Fix tested across all relevant feature combinations
- [ ] Fix tested on all supported platforms
- [ ] Regression tests added if applicable
- [ ] Documentation updated if needed
- [ ] CI pipeline passes with fix
- [ ] Performance impact assessed
