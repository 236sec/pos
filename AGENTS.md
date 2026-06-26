Do not make any changes until you have 95% confidence in what you need to build. Ask me follow-up questions until you reach that confidence.
be brief.
When talking to me, sacrifice grammar for the sake of concision.

## Strict Git Protocol

You are operating inside a live Git repository. You do not have permission to make messy, untracked, or direct changes. Whenever you are assigned a task that modifies code.
you must using git worktree with git branch to operate parallel branches.
naming convention: `[type]/[agent-name]-[short-kebab-case-slug]`

- Allowed types: `feat`, `fix`, `refactor`, `docs`, `test`, `style`
- Example: `git checkout -b feat/database-agent-add-users-table`

Write commit messages adhering strictly to the **Conventional Commits** specification:
`<type>(<scope>): <imperative, lowercase description>`

- _Good:_ `feat(auth): implement bcrypt password hashing`
- _Bad:_ `Added some hashing to the password file`

* **FORBIDDEN COMMANDS:** `git add .`, `git add -A`, and `git commit -a`.
* You must stage files **explicitly by name** to prevent accidentally tracking system files, IDE caches, or `.env` secrets.
* Before committing, run `git status` to verify your staged list.

### Hand-off Protocol

When you finish a task, you must push your branch to the remote repository and create a pull request for review. Use the following commands:

1. `git status` (Ensure working tree is clean)
2. `git push -u origin <your-branch-name>`
