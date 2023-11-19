# Git Conventional Commit

A wrapper around `git commit` that makes use of the [Conventional Commits](https://www.conventionalcommits.org) spec.

Created simply for some Rust fun.

---

## How It Works

### Method 1: Command Line Arguments

One method to using this is by using command line args on their own. There are multiple available:

- `-m`, `--message` - Add a message to the commit [REQUIRED]
- `-t`, `--type` - Add a type to the commit (fix, feat, etc.) [REQUIRED]
- `-s`, `--scope` - Add a scope to the commit
- `-b`, `--body` - Add a body to the commit (can be chained)
- `-f`, `--footer` - Add a footer to the commit
- `-a`, `--amend` - Amend the previous commit
- `--breaking` - Indicate a breaking change
- `--all` - `git add .` automatically

I would also like to add zsh autocomplete functionality, so things like `--type` can get autocompleted.

#### Example:
```bash
/path/to/git-conventional-commit -m "This is a commit" -t "fix" -s "some_scope"
# git commit -m "fix(some_scope): This is a commit"
```

### Method 2: Interactive

This method will just have you type `/path/to/git-conventional-commit`.

The script will then walk you through creating a new commit:

```
What type of commit is this?: fix
What is the commit message?: This is a commit
What is the scope of this commit? (Optional): some_scope
What is the body of this commit? (// = new paragraph) (Optional): A body // Some content
Do you want to commit all files? (y/N): y
What is the footer of this commit? (Optional): Some footer here
Is this a breaking change? (y/N): n
```

# Why Rust?

Because this script is 🔥🔥🔥🔥BLAZINGLY FAST!!!!!!!!!🔥🔥🔥🔥

No.

I'm trying to learn Rust, I hate shell scripting with a burning passion, and I think doing simple scripts like this will not only help me learn Rust, but also help me make actually good commit messages.

