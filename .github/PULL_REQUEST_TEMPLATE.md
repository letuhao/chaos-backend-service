# Pull Request

## Description
<!-- Provide a brief description of what this PR does -->

## Type of Change
<!-- Mark the relevant option with an "x" -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Performance improvement
- [ ] Code refactoring (no functional changes)
- [ ] Documentation update
- [ ] Configuration change
- [ ] CI/CD improvement
- [ ] Test improvement

## Related Issues
<!-- Link to related issues -->
- Fixes #
- Related to #
- Depends on #

## Changes Made
<!-- Describe the changes in detail -->

### Core Changes
- 
- 
- 

### Configuration Changes
- 
- 

### Documentation Changes
- 
- 

## Feature Impact
<!-- Which features does this change affect? -->

- [ ] Default features (core functionality)
- [ ] `moka-cache` feature
- [ ] `memory-mapped` feature  
- [ ] `redis-cache` feature
- [ ] `mongodb-storage` feature
- [ ] `sqlx-database` feature
- [ ] `cli-tools` feature
- [ ] `heavy-deps` feature combination
- [ ] All features

## Testing
<!-- Describe how this change has been tested -->

### Local Testing
- [ ] `cargo test -p actor-core` (default features)
- [ ] `cargo test -p actor-core --all-features` (all features)
- [ ] `cargo test -p actor-core --test '*' --all-features` (integration tests)
- [ ] `make test-feature-matrix` (feature combinations)
- [ ] Examples run successfully
- [ ] Benchmarks run without regression

### CI Requirements
<!-- These will be automatically checked by CI -->
- [ ] All feature matrix builds pass
- [ ] Code quality checks pass (clippy, fmt)
- [ ] Security audit passes
- [ ] Cross-platform tests pass
- [ ] Performance tests pass
- [ ] Documentation builds without warnings
- [ ] Integration tests pass

## Performance Impact
<!-- Assess the performance impact of your changes -->

- [ ] No performance impact expected
- [ ] Performance improvement expected
- [ ] Minor performance impact (< 5%)
- [ ] Significant performance impact (> 5%)
- [ ] Performance impact unknown - needs benchmarking

### Benchmark Results
<!-- If performance impact is expected, provide benchmark results -->

```
[Paste benchmark comparison here if applicable]
```

## Breaking Changes
<!-- If this introduces breaking changes, describe them -->

### API Changes
- 
- 

### Configuration Changes
- 
- 

### Migration Guide
<!-- How should users migrate their code? -->

1. 
2. 
3. 

## Checklist
<!-- Mark completed items with an "x" -->

### Code Quality
- [ ] Code follows the project's style guidelines
- [ ] Self-review of code completed
- [ ] Code is properly commented, particularly in hard-to-understand areas
- [ ] No debug prints or commented-out code left behind
- [ ] Error handling is appropriate and consistent

### Testing
- [ ] New tests added for new functionality
- [ ] Existing tests updated if needed
- [ ] Edge cases considered and tested
- [ ] Integration tests cover the changes
- [ ] Performance tests added if applicable

### Documentation
- [ ] Documentation updated for new features
- [ ] API documentation updated
- [ ] Examples updated if needed
- [ ] CHANGELOG.md updated
- [ ] README.md updated if needed

### Configuration
- [ ] Configuration files validated
- [ ] Schema updates if applicable
- [ ] Migration scripts provided if needed
- [ ] Backward compatibility maintained where possible

### Dependencies
- [ ] New dependencies justified and minimal
- [ ] Dependency versions pinned appropriately
- [ ] Feature flags used for optional dependencies
- [ ] Security implications of dependencies considered

## Deployment Considerations
<!-- Any special considerations for deployment -->

- [ ] No special deployment requirements
- [ ] Configuration changes needed
- [ ] Database migrations required
- [ ] Service restart required
- [ ] Rollback plan documented

## Additional Notes
<!-- Any additional information for reviewers -->

### Review Focus Areas
<!-- What should reviewers pay special attention to? -->

- 
- 
- 

### Known Limitations
<!-- Any known limitations or technical debt introduced -->

- 
- 

### Future Work
<!-- Related work that should be done in future PRs -->

- 
- 

## Screenshots/Logs
<!-- If applicable, add screenshots or log outputs -->

## Reviewer Assignment
<!-- Tag specific reviewers if needed -->

- @<!-- username --> for <!-- reason -->
- @<!-- username --> for <!-- reason -->

---

**By submitting this PR, I confirm that:**
- [ ] I have read and understood the contribution guidelines
- [ ] My code follows the project's coding standards
- [ ] I have tested my changes thoroughly
- [ ] I have updated documentation as needed
- [ ] I understand that this PR will be subject to CI checks before merging
