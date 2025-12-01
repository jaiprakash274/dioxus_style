# Publishing Checklist for crates.io

Complete guide for publishing dioxus_style v0.2.0 to crates.io.

## Pre-Publishing Checklist

### 1. Code Quality ✅

- [ ] All tests pass: `cargo test --all`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code is formatted: `cargo fmt --all -- --check`
- [ ] Documentation builds: `cargo doc --no-deps --all-features`
- [ ] Element scoping tests pass
- [ ] Complex selector tests pass
- [ ] Minification tests pass

### 2. Documentation ✅

- [ ] README.md is updated for v0.2.0
  - [ ] Element scoping examples added
  - [ ] `data-scope` attribute usage documented
  - [ ] Migration guide from v0.1.0 included
  - [ ] New scoping behavior explained
- [ ] CHANGELOG.md is updated with v0.2.0 changes
  - [ ] Breaking changes listed
  - [ ] New features documented
  - [ ] Bug fixes noted
- [ ] All public APIs have doc comments
- [ ] Examples in docs are working and updated
- [ ] LICENSE-MIT and LICENSE-APACHE files exist

### 3. Cargo.toml Configuration ✅

Check all Cargo.toml files have correct metadata:

#### Workspace Cargo.toml
```toml
[workspace.package]
version = "0.2.0"  # ⚠️ UPDATED VERSION
edition = "2021"
authors = ["JAI PRAKASH THAWAIT <jaiprakashthawait@gmail.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/jaiprakash274/dioxus_style"
```

#### dioxus_style/Cargo.toml
- [ ] Version = "0.2.0"
- [ ] Description mentions element scoping
- [ ] Keywords include "scoping", "css", "dioxus"
- [ ] Categories are appropriate
- [ ] Dependencies are correct

#### dioxus_style_macro/Cargo.toml
- [ ] Version = "0.2.0"
- [ ] Description is clear
- [ ] Keywords are relevant
- [ ] `proc-macro = true` is set

### 4. Repository Setup ✅

- [ ] Git repository is up to date
- [ ] All changes are committed
- [ ] Repository is pushed to GitHub
- [ ] Repository URL in Cargo.toml is correct
- [ ] .gitignore includes `/target/`, `Cargo.lock`
- [ ] Branch is `main` or appropriate

### 5. Version 0.2.0 Specific Checks

- [ ] Element scoping implementation is complete
- [ ] `data-scope` attribute requirement is documented
- [ ] Migration guide from v0.1.0 is clear
- [ ] Breaking changes are explicitly stated
- [ ] All selector types are tested (class, ID, element)
- [ ] Complex selector handling is verified
- [ ] Examples show new `data-scope` usage

### 6. Testing Checklist

```bash
# Run all tests
cargo test --all

# Test element scoping specifically
cargo test -p dioxus_style_macro test_element_selector
cargo test -p dioxus_style_macro test_mixed_selector

# Test class scoping format
cargo test -p dioxus_style_macro test_class_selector_scoping

# Test ID scoping format
cargo test -p dioxus_style_macro test_id_selector_scoping

# Verify no regressions
cargo test --all -- --nocapture
```

### 7. README badges (optional) 🔜

Add these to README.md after publishing:

```markdown
[![Crates.io](https://img.shields.io/crates/v/dioxus_style.svg)](https://crates.io/crates/dioxus_style)
[![Documentation](https://docs.rs/dioxus_style/badge.svg)](https://docs.rs/dioxus_style)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/dioxus_style.svg)](https://crates.io/crates/dioxus_style)
```

## Publishing Steps

### Step 1: Verify You're Logged In

```bash
# Check if you're logged in
cargo login --help

# If not logged in, get token from https://crates.io/settings/tokens
cargo login <your-token-here>
```

### Step 2: Verify Package (Dry Run)

Test packaging without publishing:

```bash
# Test macro package first (dependency)
cd dioxus_style_macro
cargo package --list
cargo package --allow-dirty  # if you have uncommitted changes

# Verify contents
tar -tzf target/package/dioxus_style_macro-0.2.0.crate

# Test main package
cd ../dioxus_style
cargo package --list
cargo package --allow-dirty

# Verify contents
tar -tzf target/package/dioxus_style-0.2.0.crate
```

### Step 3: Final Pre-Publish Checks

```bash
# Build documentation locally
cargo doc --no-deps --all-features --open

# Check for any warnings
cargo build --release --all

# Run clippy one more time
cargo clippy --all-targets --all-features -- -D warnings
```

### Step 4: Publish Packages

**IMPORTANT**: Publish in dependency order!

```bash
# 1. Publish macro package FIRST (it's a dependency)
cd dioxus_style_macro
cargo publish

# Wait for it to be available (check https://crates.io/crates/dioxus_style_macro)
# This may take 5-10 minutes for the index to update

# 2. Then publish main package
cd ../dioxus_style
cargo publish
```

### Step 5: Verify Publication

1. Check on crates.io:
   - https://crates.io/crates/dioxus_style (should show v0.2.0)
   - https://crates.io/crates/dioxus_style_macro (should show v0.2.0)

2. Wait for docs to build (10-15 minutes):
   - https://docs.rs/dioxus_style/0.2.0
   - https://docs.rs/dioxus_style_macro/0.2.0

3. Test installation in a fresh project:
```bash
cargo new test_dioxus_style_v02
cd test_dioxus_style_v02
cargo add dioxus_style@0.2.0
cargo check
```

4. Test the new features:
```bash
# Create test file to verify element scoping works
cat > src/main.rs << 'EOF'
use dioxus::prelude::*;
use dioxus_style::with_css;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TestComponent {}
    }
}

#[with_css("test.css")]
fn TestComponent() -> Element {
    rsx! {
        div {
            "data-scope": "{css}",
            class: "{css}_container",
            "Test v0.2.0 element scoping"
        }
    }
}
EOF

# Create test CSS
cat > test.css << 'EOF'
.container { padding: 20px; }
div { margin: 10px; }
EOF

cargo check
```

## Post-Publishing Tasks

### 1. Create Git Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0 - Element Scoping"
git push origin v0.2.0
```

### 2. Create GitHub Release

1. Go to repository → Releases → Create new release
2. Choose tag: v0.2.0
3. Title: "v0.2.0 - Element Scoping Release"
4. Description:
```markdown
## 🎉 dioxus_style v0.2.0 - Element Scoping Release

### ✨ New Features
- **Element Selector Scoping**: Elements like `div`, `span`, `p` are now automatically scoped using `data-scope` attributes
- Enhanced ID selector scoping format
- Improved complex selector handling

### ⚠️ Breaking Changes
- Element selectors now require `data-scope` attribute on elements
- Class selector output format changed from `.sc_xxx.class` to `.sc_xxx_class`
- ID selector output format changed from `#sc_xxx.id` to `#sc_xxx_id`

### 📝 Migration Guide
See [CHANGELOG.md](https://github.com/jaiprakash274/dioxus_style/blob/main/CHANGELOG.md#020---2025-12-01) for detailed migration instructions.

### 📦 Installation
```toml
[dependencies]
dioxus_style = "0.2.0"
```

### 📚 Documentation
- [docs.rs](https://docs.rs/dioxus_style/0.2.0)
- [crates.io](https://crates.io/crates/dioxus_style)

### 🙏 Thanks
Thank you to all contributors and users!
```
5. Publish release

### 3. Announce Release (Optional)

- [ ] Post in Dioxus Discord #showcase channel
- [ ] Post on Reddit r/rust with migration guide
- [ ] Tweet about the new features
- [ ] Update any blog posts or articles

### 4. Update Repository

- [ ] Add crates.io v0.2.0 badges to README
- [ ] Update documentation links to point to v0.2.0 docs
- [ ] Create "Next Version" section in CHANGELOG
- [ ] Update any example projects to use v0.2.0

### 5. Monitor for Issues

- [ ] Watch GitHub issues for bug reports
- [ ] Monitor crates.io downloads
- [ ] Check docs.rs build status
- [ ] Respond to questions in discussions

## Common Issues & Solutions

### Issue: "crate version already published"

**Solution**: Cannot republish. Must bump version:
```bash
# Update to 0.2.1 or 0.3.0
# Update all Cargo.toml files
# Update CHANGELOG.md
# Commit and retry
```

### Issue: "failed to verify package"

**Solution**: 
```bash
cargo package --list  # Check what files are included
cargo package --allow-dirty  # If you have uncommitted changes

# Make sure test.css or example CSS files are included
# Check Cargo.toml [package] section for 'include' or 'exclude'
```

### Issue: "dependency not found"

**Solution**: 
1. Ensure `dioxus_style_macro` v0.2.0 is published first
2. Wait 5-10 minutes for crates.io index to update
3. Check https://crates.io/crates/dioxus_style_macro/0.2.0
4. Then publish main crate

### Issue: "documentation failed to build"

**Solution**:
- Build docs locally: `cargo doc --no-deps`
- Fix any doc warnings
- Ensure all examples compile
- Check for missing dependencies

### Issue: "Version 0.2.0 already published"

**Solution**: You cannot republish. Options:
1. Yank the broken version: `cargo yank --vers 0.2.0`
2. Publish a patch: bump to 0.2.1
3. Wait for next release cycle

## Version Bumping Guide

For future releases:

### Patch (0.2.0 → 0.2.1)
- Bug fixes only
- Documentation improvements
- Performance improvements (non-breaking)
- No API changes

### Minor (0.2.0 → 0.3.0)
- New features
- Non-breaking API additions
- Deprecations (with warnings)
- Behavioral changes (documented)

### Major (0.2.0 → 1.0.0)
- Breaking API changes
- Removed deprecated features
- Major refactoring
- Incompatible behavioral changes

## Quick Command Reference

```bash
# Pre-publish checks
cargo test --all && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo fmt --all -- --check && \
cargo doc --no-deps --all-features

# Package verification
cd dioxus_style_macro && cargo package --list
cd ../dioxus_style && cargo package --list

# Publish (wait between steps!)
cd dioxus_style_macro && cargo publish
# ⏰ Wait 5-10 minutes
cd ../dioxus_style && cargo publish

# Create release
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# Verify installation
cargo new test_v02
cd test_v02
cargo add dioxus_style@0.2.0
cargo check
```

## Pre-Publish Commands Summary (v0.2.0)

Run these before publishing:

```bash
# 1. Ensure you're on latest main
git checkout main
git pull origin main

# 2. Run all tests
cargo test --all

# 3. Check for warnings
cargo clippy --all-targets --all-features -- -D warnings

# 4. Format code
cargo fmt --all

# 5. Build documentation
cargo doc --no-deps --all-features --open

# 6. Verify packages
cd dioxus_style_macro && cargo package --list
cd ../dioxus_style && cargo package --list

# 7. Commit everything
git add -A
git commit -m "Release v0.2.0 - Element Scoping"
git push origin main

# 8. Publish macro first
cd dioxus_style_macro
cargo publish

# 9. Wait 5-10 minutes and verify
# Check: https://crates.io/crates/dioxus_style_macro/0.2.0

# 10. Publish main crate
cd ../dioxus_style
cargo publish

# 11. Tag release
git tag -a v0.2.0 -m "Release v0.2.0 - Element Scoping"
git push origin v0.2.0

# 12. Verify installation
cd /tmp
cargo new test_dioxus_style_v02
cd test_dioxus_style_v02
cargo add dioxus_style@0.2.0
cargo check
```

## Support & Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io status](https://status.crates.io/)
- [Rust Users Forum](https://users.rust-lang.org/)
- [Dioxus Discord](https://discord.gg/XgGxMSkvUM)

---

**Good luck with v0.2.0 publish! 🚀**

**New in this version**: Element scoping with `data-scope` attributes!