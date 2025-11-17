# Publishing Checklist for crates.io

Complete guide for publishing dioxus_style to crates.io.

## Pre-Publishing Checklist

### 1. Code Quality ✅

- [ ] All tests pass: `cargo test --all`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code is formatted: `cargo fmt --all -- --check`
- [ ] Documentation builds: `cargo doc --no-deps --all-features`

### 2. Documentation ✅

- [ ] README.md is complete and accurate
- [ ] CHANGELOG.md is updated with version 0.1.0
- [ ] All public APIs have doc comments
- [ ] Examples in docs are working
- [ ] LICENSE-MIT and LICENSE-APACHE files exist

### 3. Cargo.toml Configuration ✅

Check all Cargo.toml files have correct metadata:

#### Workspace Cargo.toml
```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["JAI PRAKASH THAWAIT <jaiprakashthawait@gmail.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/jaiprakash274/dioxus_style"  # ⚠️ UPDATE THIS
```

#### dioxus_style/Cargo.toml
- [ ] Description is clear and concise
- [ ] Keywords are relevant (max 5)
- [ ] Categories are appropriate
- [ ] Version matches workspace

#### dioxus_style_macro/Cargo.toml
- [ ] Description is clear
- [ ] Keywords are relevant
- [ ] `proc-macro = true` is set
- [ ] Version matches workspace

### 4. Repository Setup ✅

- [ ] Git repository is initialized
- [ ] All changes are committed
- [ ] Repository is pushed to GitHub/GitLab
- [ ] Repository URL in Cargo.toml is correct
- [ ] .gitignore includes `/target/`, `Cargo.lock` (for libraries)

### 5. README badges (optional) 📛

Add these to README.md after publishing:

```markdown
[![Crates.io](https://img.shields.io/crates/v/dioxus_style.svg)](https://crates.io/crates/dioxus_style)
[![Documentation](https://docs.rs/dioxus_style/badge.svg)](https://docs.rs/dioxus_style)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/dioxus_style.svg)](https://crates.io/crates/dioxus_style)
```

## Publishing Steps

### Step 1: Create crates.io Account

1. Go to https://crates.io/
2. Click "Log in with GitHub"
3. Authorize the application

### Step 2: Get API Token

1. Go to https://crates.io/settings/tokens
2. Click "New Token"
3. Name it (e.g., "dioxus_style_publishing")
4. Copy the token

### Step 3: Login to Cargo

```bash
cargo login <your-token-here>
```

This saves your token to `~/.cargo/credentials.toml`

### Step 4: Verify Package (Dry Run)

Test packaging without publishing:

```bash
# Test macro package first (dependency)
cd dioxus_style_macro
cargo package --list
cargo package --allow-dirty  # if you have uncommitted changes

# Test main package
cd ../dioxus_style
cargo package --list
cargo package --allow-dirty
```

### Step 5: Publish Packages

**IMPORTANT**: Publish in dependency order!

```bash
# 1. Publish macro package FIRST (it's a dependency)
cd dioxus_style_macro
cargo publish

# Wait for it to be available (check https://crates.io/crates/dioxus_style_macro)
# This may take a few minutes

# 2. Then publish main package
cd ../dioxus_style
cargo publish
```

### Step 6: Verify Publication

1. Check on crates.io:
   - https://crates.io/crates/dioxus_style
   - https://crates.io/crates/dioxus_style_macro

2. Wait for docs to build (5-10 minutes):
   - https://docs.rs/dioxus_style
   - https://docs.rs/dioxus_style_macro

3. Test installation:
```bash
cargo new test_project
cd test_project
cargo add dioxus_style
cargo check
```

## Post-Publishing Tasks

### 1. Create Git Tag

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### 2. Create GitHub Release

1. Go to repository → Releases → Create new release
2. Choose tag: v0.1.0
3. Title: "v0.1.0 - Initial Release"
4. Description: Copy from CHANGELOG.md
5. Publish release

### 3. Announce Release (Optional)

- [ ] Post in Dioxus Discord
- [ ] Tweet about it
- [ ] Post on Reddit r/rust
- [ ] Update personal website/blog

### 4. Update Repository

- [ ] Add crates.io badges to README
- [ ] Update documentation links
- [ ] Create "Next Version" section in CHANGELOG

## Common Issues & Solutions

### Issue: "crate name is already taken"

**Solution**: Choose a different name. Try variations like:
- `dioxus-style`
- `dioxus_scoped_style`
- `dx_style`

### Issue: "failed to verify package"

**Solution**: 
```bash
cargo package --list  # Check what files are included
cargo package --allow-dirty  # If you have uncommitted changes
```

### Issue: "dependency not found"

**Solution**: Make sure you published dependencies first. For this project:
1. Publish `dioxus_style_macro` first
2. Wait a few minutes
3. Then publish `dioxus_style`

### Issue: "documentation failed to build"

**Solution**:
- Check docs build locally: `cargo doc --no-deps`
- Fix any doc warnings
- Ensure all examples in doc comments compile

### Issue: "Version already published"

**Solution**: You cannot republish the same version. Must bump version:
1. Update version in Cargo.toml files
2. Update CHANGELOG.md
3. Commit changes
4. Publish new version

## Version Bumping Guide

For future releases:

### Patch (0.1.0 → 0.1.1)
- Bug fixes
- Documentation improvements
- Performance improvements

### Minor (0.1.0 → 0.2.0)
- New features
- Non-breaking API additions
- Deprecations

### Major (0.1.0 → 1.0.0)
- Breaking API changes
- Major refactoring
- Removed deprecated features

## Quick Command Reference

```bash
# Check everything
cargo test --all && cargo clippy --all-targets -- -D warnings && cargo fmt --all -- --check

# Package verification
cargo package --list

# Publish (in order)
cd dioxus_style_macro && cargo publish
cd ../dioxus_style && cargo publish

# Create release
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Verify installation
cargo new test_project
cd test_project
cargo add dioxus_style
cargo check
```

## Pre-Publish Commands Summary

Run these before publishing:

```bash
# 1. Run all tests
cargo test --all

# 2. Check for warnings
cargo clippy --all-targets --all-features -- -D warnings

# 3. Format code
cargo fmt --all

# 4. Build documentation
cargo doc --no-deps --all-features --open

# 5. Verify packages
cd dioxus_style_macro && cargo package --list
cd ../dioxus_style && cargo package --list

# 6. Commit everything
git add -A
git commit -m "Prepare for v0.1.0 release"
git push

# 7. Publish
cd dioxus_style_macro && cargo publish
# Wait 2-5 minutes
cd ../dioxus_style && cargo publish

# 8. Tag release
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

## Support

If you encounter issues:
- Check [crates.io status](https://status.crates.io/)
- Read [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- Ask in [Rust Users Forum](https://users.rust-lang.org/)

---

**Good luck with your publish! 🚀**