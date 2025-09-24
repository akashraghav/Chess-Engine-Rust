# Pull Request

## 📋 Description

<!-- Provide a brief description of what this PR does -->

**Type of Change:**
- [ ] 🐛 Bug fix (non-breaking change that fixes an issue)
- [ ] ✨ New feature (non-breaking change that adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 📚 Documentation update (improvements to docs, comments, examples)
- [ ] ⚡ Performance improvement (non-breaking change that improves performance)
- [ ] 🧹 Code refactoring (non-functional change that improves code structure)
- [ ] 🧪 Test improvements (adding or improving tests)

## 🔗 Related Issues

<!-- Link to related issues using "Fixes #123" or "Closes #123" syntax -->

- Fixes #
- Related to #

## 🔧 Changes Made

<!-- Describe the changes in detail -->

### Core Changes
-
-
-

### API Changes (if applicable)
-
-

### Performance Impact (if applicable)
-
-

## 🧪 Testing

<!-- Describe how you tested your changes -->

**Test Categories Completed:**
- [ ] ✅ Unit tests pass (`cargo test`)
- [ ] 🧪 Integration tests pass
- [ ] ⚡ Performance tests/benchmarks (if applicable)
- [ ] 🎯 Chess rules validation tests
- [ ] 📱 Platform-specific tests (if applicable)

**Manual Testing:**
- [ ] Tested on multiple platforms (specify which ones)
- [ ] Tested with different engine configurations
- [ ] Tested edge cases and error conditions

**New Tests Added:**
- [ ] Unit tests for new functionality
- [ ] Integration tests for feature interactions
- [ ] Performance benchmarks (if performance-related)
- [ ] Chess-specific rule tests (if rules-related)

## 📊 Performance Impact

<!-- Fill out if this PR affects performance -->

**Benchmarks Results:**

<details>
<summary>Click to expand benchmark data</summary>

```
Before:
- Move generation: X moves/second
- Search (depth 6): Y nodes/second
- Memory usage: Z MB

After:
- Move generation: A moves/second (+/- B%)
- Search (depth 6): C nodes/second (+/- D%)
- Memory usage: E MB (+/- F MB)
```

</details>

## 🎯 Chess Engine Specific

<!-- Fill out if this affects chess functionality -->

**Chess Rules/Features Affected:**
- [ ] Move generation (normal moves)
- [ ] Special moves (castling, en passant, promotion)
- [ ] Position evaluation
- [ ] Search algorithms
- [ ] Game state management
- [ ] FEN/PGN parsing
- [ ] Draw detection (threefold, fifty-move, insufficient material)
- [ ] Check/checkmate detection

**Validation:**
- [ ] Tested against known chess positions
- [ ] Verified against chess rule specifications
- [ ] Cross-referenced with other chess engines (if applicable)

## 📚 Documentation

**Documentation Updated:**
- [ ] Code comments added/updated
- [ ] API documentation updated
- [ ] README.md updated (if needed)
- [ ] HOW-IT-WORKS.md updated (if architecture changes)
- [ ] Examples updated/added
- [ ] Changelog updated (CHANGELOG.md)

## 🔍 Code Quality

**Code Quality Checklist:**
- [ ] 🎨 Code follows the style guidelines (`cargo fmt`)
- [ ] 🔍 No clippy warnings (`cargo clippy`)
- [ ] 📖 Functions and public APIs are documented
- [ ] ⚠️ Error handling is appropriate
- [ ] 🧪 Test coverage is maintained/improved
- [ ] 🏗️ Follows project architecture guidelines

## 🚀 Deployment Checklist

<!-- For maintainers -->

**Pre-merge Checklist:**
- [ ] All CI checks pass
- [ ] Code review completed
- [ ] Performance impact assessed
- [ ] Breaking changes documented
- [ ] Version bump needed (if applicable)

## 💡 Additional Notes

<!-- Any additional information, context, or screenshots -->

### Implementation Details

<!-- Explain any complex implementation decisions -->

### Future Work

<!-- Any follow-up work or improvements identified -->

### Questions for Reviewers

<!-- Specific areas you'd like feedback on -->

---

## 🏆 Reviewer Guidelines

**For Reviewers:**

1. **Functionality**: Does it work correctly? Are edge cases handled?
2. **Performance**: Any performance regressions? Are optimizations sound?
3. **Chess Rules**: Are chess rules implemented correctly?
4. **Code Quality**: Is the code clean, readable, and maintainable?
5. **Testing**: Is test coverage adequate? Are tests meaningful?
6. **Documentation**: Are public APIs documented? Is usage clear?

**Review Focus Areas:**
- [ ] Algorithm correctness (especially for chess-related changes)
- [ ] Performance impact (benchmark any performance claims)
- [ ] API design (for new public interfaces)
- [ ] Error handling (edge cases and failure modes)
- [ ] Test quality (comprehensive and maintainable)

---

**By submitting this PR, I confirm that:**

- [ ] I have read and followed the [Contributing Guidelines](../CONTRIBUTING.md)
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes