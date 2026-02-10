# Contributing to Censor

Thank you for your interest in contributing to Censor! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Report security vulnerabilities privately

## How to Contribute

### Reporting Issues

1. **Check existing issues** to avoid duplicates
2. **Provide detailed information**:
   - OS and version
   - Application version
   - Steps to reproduce
   - Expected vs actual behavior
   - Logs or error messages
   - Screenshots if applicable

### Suggesting Features

1. **Describe the use case** - Why is this needed?
2. **Explain the expected behavior**
3. **Consider alternatives** - What else could work?
4. **Check for security implications**

### Code Contributions

#### Getting Started

1. **Fork the repository**
2. **Clone your fork**
   ```bash
   git clone https://github.com/your-username/censor.git
   cd censor
   ```

3. **Create feature branch**
   ```bash
   git checkout -b feature/descriptive-name
   ```

4. **Set up development environment**
   ```bash
   npm install
   npm run tauri:dev
   ```

#### Development Workflow

1. **Make changes** following code style guidelines (see below)
2. **Test thoroughly**
   ```bash
   npm run build
   npm run tauri:build
   ```

3. **Commit with clear messages**
   ```bash
   git commit -m "feat: add feature description"
   # or
   git commit -m "fix: resolve issue description"
   git commit -m "docs: update documentation"
   ```

4. **Push to your fork**
   ```bash
   git push origin feature/descriptive-name
   ```

5. **Create Pull Request**
   - Describe changes clearly
   - Link related issues
   - Request review

#### Commit Message Format

Use conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `style:` - Code style (formatting, semicolons, etc.)
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Adding/updating tests
- `chore:` - Build, dependency updates

Example:
```
feat: add motion blur censoring method

- Implement motion blur algorithm
- Add motion blur to CensoringMethod enum
- Update UI to display motion blur option
- Add unit tests for motion blur

Fixes #123
```

## Code Style Guidelines

### Rust
- Follow `rustfmt` defaults
- Use meaningful variable names
- Add documentation comments for public functions
- Avoid `unwrap()` - use `Result` types
- Use appropriate error handling

```rust
/// Applies Gaussian blur to masked region
pub fn apply_blur(&mut image: RgbaImage, polygon: &[(f64, f64)]) -> Result<(), String> {
    // Implementation
}
```

### TypeScript/Vue
- Follow ESLint configuration
- Use TypeScript for type safety
- Prefer `const` over `let`, avoid `var`
- Use meaningful names

```typescript
const detectContent = async (imageBase64: string, locale: string): Promise<Region[]> => {
    // Implementation
}
```

### Vue Components
- Use composition API `<script setup>`
- Split large components into smaller ones
- Document complex logic with comments
- Use scoped styles

```vue
<script setup lang="ts">
import { ref } from 'vue'

const isOpen = ref(false)
</script>
```

## Testing

### Before Submitting PR

1. **Manual Testing Checklist**:
   - [ ] Image upload works
   - [ ] Detection completes successfully
   - [ ] Regions display correctly
   - [ ] Masks can be adjusted
   - [ ] All censoring methods work
   - [ ] Save/load functions work
   - [ ] Remote Ollama connection works

2. **Performance Testing**:
   - [ ] Test with large images (5000x5000+)
   - [ ] Test with many regions (100+)
   - [ ] Check CPU/memory usage

## Documentation

- Update README.md for user-facing changes
- Update DEVELOPMENT.md for technical changes
- Update ARCHITECTURE.md for structural changes
- Add inline code comments for complex logic
- Document new APIs/commands

## Pull Request Process

1. **Ensure your branch is up to date**
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Verify all tests pass**
   ```bash
   npm run build
   npm run tauri:build
   ```

3. **Create descriptive PR**
   - Title: Clear and concise (50 chars max)
   - Description: Explain what and why
   - Screenshots for UI changes
   - Link related issues

4. **Respond to reviews** - Be open to feedback

5. **Keep commits clean** - Squash minor fixes

## Adding New Features

### Detection Type

1. Update `models.rs`:
   ```rust
   pub enum DetectionType {
       YourNewType,  // Add here
   }
   ```

2. Update `ollama.rs` with appropriate prompt

3. Update `config.rs` if needed

4. Update frontend settings UI

### Censoring Method

1. Implement in `image_processor.rs`
2. Add to `CensoringMethod` enum
3. Update `commands.rs` handler
4. Update UI method selector

### Locale

1. Add to `config.rs` locale list
2. Define nudity standards
3. Update UI dropdown

## Performance Considerations

- Image processing should handle large files efficiently
- Avoid blocking the UI thread
- Use async/await for I/O operations
- Cache frequently used data
- Profile before/after optimization

## Security

- Never expose API keys or credentials
- Validate all user inputs
- Be careful with file operations
- Keep dependencies updated
- Report security issues privately

## Release Process

Only maintainers can create releases:

1. Update version in `package.json` and `Cargo.toml`
2. Update CHANGELOG.md
3. Create git tag: `git tag v1.x.x`
4. Push and create GitHub release
5. Upload compiled binaries

## Questions?

- Check [DEVELOPMENT.md](DEVELOPMENT.md)
- Check [ARCHITECTURE.md](ARCHITECTURE.md)
- Open an issue for discussion
- Reach out to maintainers

## License

By contributing, you agree your work is licensed under MIT License.

---

Thank you for making Censor better! 🎉
