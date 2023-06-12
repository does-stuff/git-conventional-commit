# Git Conventional Commit

A wrapper around `git commit` that makes use of the [Conventional Commits](https://www.conventionalcommits.org) spec.

## The Idea

```sh
# `gc` acts as an alias to this script in this example
# `--body` can be chained to add multiple paragraphs
gc --type "feat" --scope "kitty" --description "Added background opacity" --body "Insert body here..." --breaking "This somehow completely ruins the configuration or something"

# ... or
gc -t "feat" -s "kitty" -d "Added background opacity" -b "Insert body here..." --breaking "This somehow completely ruins the configuration or something"

# feat(kitty)!: Added background opacity
#
# Insert body here...
#
# BREAKING CHANGE: This somehow completely ruins the configuration or something
```

I would also like to add zsh autocomplete functionality, so things like `--type` can get autocompleted.

# Why Rust?

Because this script is 🔥🔥🔥🔥BLAZINGLY FAST!!!!!!!!!🔥🔥🔥🔥

No.

I'm trying to learn Rust, I hate shell scripting with a burning passion, and I think doing simple scripts like this will not only help me learn Rust, but also help me make actually good commit messages.

# Specification

1. Commits MUST be prefixed with a type, which consists of a noun, feat, fix, etc., followed by the OPTIONAL scope, OPTIONAL !, and REQUIRED terminal colon and space.
2. The type feat MUST be used when a commit adds a new feature to your application or library.
3. The type fix MUST be used when a commit represents a bug fix for your application.
4. A scope MAY be provided after a type. A scope MUST consist of a noun describing a section of the codebase surrounded by parenthesis, e.g., fix(parser):
5. A description MUST immediately follow the colon and space after the type/scope prefix. The description is a short summary of the code changes, e.g., fix: array parsing issue when multiple spaces were contained in string.
6. A longer commit body MAY be provided after the short description, providing additional contextual information about the code changes. The body MUST begin one blank line after the description.
7. A commit body is free-form and MAY consist of any number of newline separated paragraphs.
8. One or more footers MAY be provided one blank line after the body. Each footer MUST consist of a word token, followed by either a :<space> or <space># separator, followed by a string value (this is inspired by the git trailer convention).
9. A footer’s token MUST use - in place of whitespace characters, e.g., Acked-by (this helps differentiate the footer section from a multi-paragraph body). An exception is made for BREAKING CHANGE, which MAY also be used as a token.
10. A footer’s value MAY contain spaces and newlines, and parsing MUST terminate when the next valid footer token/separator pair is observed.
11. Breaking changes MUST be indicated in the type/scope prefix of a commit, or as an entry in the footer.
12. If included as a footer, a breaking change MUST consist of the uppercase text BREAKING CHANGE, followed by a colon, space, and description, e.g., BREAKING CHANGE: environment variables now take precedence over config files.
13. If included in the type/scope prefix, breaking changes MUST be indicated by a ! immediately before the :. If ! is used, BREAKING CHANGE: MAY be omitted from the footer section, and the commit description SHALL be used to describe the breaking change.
14. Types other than feat and fix MAY be used in your commit messages, e.g., docs: update ref docs.
15. The units of information that make up Conventional Commits MUST NOT be treated as case sensitive by implementors, with the exception of BREAKING CHANGE which MUST be uppercase.
16. BREAKING-CHANGE MUST be synonymous with BREAKING CHANGE, when used as a token in a footer.

