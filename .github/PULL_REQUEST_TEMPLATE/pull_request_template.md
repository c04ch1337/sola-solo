# Pull Request

## Description
<!-- Provide a clear and concise description of your changes -->

## Type of Change
<!-- Mark the relevant option with an 'x' -->

- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ✨ New feature (non-breaking change which adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 📝 Documentation update
- [ ] 🧪 Test update
- [ ] 🔧 Configuration change
- [ ] ♻️ Code refactoring

## File Placement Checklist
<!-- Verify your files are in the correct locations -->

- [ ] All documentation files are in appropriate `docs/` subdirectories
- [ ] Test scripts are in `tests/scripts/`
- [ ] Build scripts are in `scripts/build/`
- [ ] Setup scripts are in `scripts/setup/`
- [ ] No loose files added to root directory (unless essential)
- [ ] Directory README.md files updated
- [ ] `DOCUMENTATION_INDEX.md` updated (if adding architecture docs)

## Documentation Updates
<!-- List any documentation you've added or updated -->

### New Documentation
- [ ] Added to correct directory
- [ ] Updated directory README.md
- [ ] Updated `DOCUMENTATION_INDEX.md` (if applicable)
- [ ] Follows naming conventions

### Modified Documentation
- [ ] Updated existing files
- [ ] Links still valid
- [ ] Cross-references updated

## Testing
<!-- Describe the tests you ran to verify your changes -->

- [ ] All existing tests pass (`cargo test --workspace`)
- [ ] New tests added for new features
- [ ] Test scripts added to `tests/scripts/` (if applicable)
- [ ] Test documentation added to `docs/testing/` (if applicable)

### Test Commands Run
```bash
# List the test commands you ran
```

## Code Quality

- [ ] Code follows project style guidelines
- [ ] Linter passes with no errors
- [ ] No warnings introduced
- [ ] Comments added for complex logic
- [ ] Public APIs documented

## Related Issues
<!-- Link to related issues -->

Fixes #(issue number)
Related to #(issue number)

## Screenshots (if applicable)
<!-- Add screenshots to help explain your changes -->

## Checklist

### Before Submitting
- [ ] I have read [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- [ ] I have followed the file placement rules
- [ ] I have updated relevant documentation
- [ ] I have updated directory README files
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes

### File Organization
- [ ] Files are in correct directories per [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- [ ] No files added to root directory (unless essential)
- [ ] Directory READMEs updated
- [ ] Master index updated (if applicable)

### Documentation
- [ ] Documentation follows naming conventions
- [ ] Links are valid and working
- [ ] Cross-references added where appropriate
- [ ] Examples provided (if applicable)

## Additional Notes
<!-- Add any additional notes or context about the PR -->

---

**For Reviewers:**
- [ ] File placement verified
- [ ] Documentation updated appropriately
- [ ] Tests pass
- [ ] Code quality acceptable
- [ ] No breaking changes (or properly documented)
