---
name: Bug report
about: Create a detailed bug report to help us improve Brack
title: 'Bug: '
labels: 'type: bug'
assignees: ''

---

### Describe the bug
Provide a clear and detailed description of the issue. Include any relevant error messages or logs that can help diagnose the problem.

### Steps to reproduce
Detail the steps needed to reproduce the bug. Make sure to be as specific as possible:

1. ...
2. ...
3. ...

### Expected behavior
Describe what you expected to happen instead of the actual outcome.

### Actual behavior
Explain what actually happened. If there are error messages, include them here.

### Component (optional)
If you know which component your issue relates to, please select it:

- [ ] CLI tools (brack)
- [ ] Tokenizer (brack-tokenizer)
- [ ] Parser (brack-parser)
- [ ] Transformer (brack-transformer)
- [ ] Macro Expander (brack-expander)
- [ ] Lower (brack-lower)
- [ ] Code Generator (brack-codegen)
- [ ] Infrastructure
- [ ] Language Server (brack-language-server)
- [ ] Project Manager (brack-project-manager)
- [ ] GitHub Actions

### Environment (please complete the following information)
- OS: [e.g., macOS, Windows, Linux]
- Brack version: [e.g., v0.1.0]
- Any additional dependencies or configurations (e.g., plugins)

### Screenshots or logs (optional)
If applicable, provide screenshots or attach logs to help illustrate the problem. If you are comfortable sharing them, this can greatly assist in identifying the issue.

### Additional context
Add any other relevant context or details that might help in resolving the issue, such as specific configurations, custom settings, or unusual usage scenarios.

### Metadata
Please run `brack version` and paste the output below.

```console
$ brack version
<output here>
```

### Steps taken to verify
Before submitting, ensure you've done the following checks:

- [ ] Confirmed that the bug is not caused by any plugins
- [ ] Searched [open bug issues] to avoid duplicates
- [ ] Reviewed [open bug pull requests] for possible solutions

[open bug issues]: https://github.com/brack-lang/brack/issues?q=is%3Aopen+is%3Aissue+label%3A%22type%3A+bug%22
[open bug pull requests]: https://github.com/brack-lang/brack/pulls?q=is%3Aopen+is%3Apr+label%3A%22type%3A+bug%22

