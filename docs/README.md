# Project documents

The roadmap lives in [GitHub issues](https://github.com/Jordan-Hall/browser/issues/1). `project-archive/` is a committed copy for reading offline or adding to a project as reference material.

## Where to start

- [Programme and ungrouped issues](project-archive/workstreams/000-programme-and-ungrouped-issues.md) covers the product direction and programme-level tasks.
- [Runtime and contracts](project-archive/workstreams/013-runtime-and-contracts.md) contains the CORE epic, requirements, individual tasks and their discussion.
- [Workstream index](project-archive/README.md) links the remaining epics.
- [Issue register](project-archive/ISSUE_REGISTER.md) lists every issue and its document location.

Use [ALL_ISSUES_AND_EPICS.txt](project-archive/ALL_ISSUES_AND_EPICS.txt) when you need a single project-context file. The [Markdown version](project-archive/ALL_ISSUES_AND_EPICS.md) contains the same material. For smaller, focused context, use the relevant workstream files instead; loading both repeats the same content.

[The PR register](project-archive/PULL_REQUEST_REGISTER.md) is a separate supplement. It preserves PR descriptions, source-head metadata and conversation comments, not code diffs or inline reviews.

## What this snapshot contains

The archive was retrieved on **18 September 2026, 19:04:57–19:05:07 UTC**:

| Material | Count |
| --- | ---: |
| Issues, including programme and requirement issues | 787 |
| Epics included in that total | 22 |
| Task issues included in that total | 674 |
| Issue-conversation comments | 270 |
| Pull requests, recorded separately | 24 |
| PR-conversation comments | 44 |

Original descriptions and comments are preserved in [snapshot.json](project-archive/snapshot.json). The documents keep source links, dates, states, checkboxes and parent references. Forty disagreements between native and body-declared parents are recorded in the archive index rather than resolved by guesswork.

GitHub may have changed since the export. A historical comment saying that something passed or was complete is still a historical comment. Use the current task PR, its tested commit and the task's acceptance criteria for implementation decisions.

## Check the archive

From the repository root:

```sh
python3 scripts/validate_project_archive.py
```

The check runs offline. It verifies the recorded file hashes, source-text hashes, counts, comment IDs and once-only issue membership. The manifest detects accidental changes; it is not a signed statement of provenance or proof of software acceptance.

Do not edit copied issue text to change project status. Make the change on GitHub, then refresh the archive. Handwritten navigation belongs here or in the root README, outside the generated snapshot.

## Refresh the archive

Use a documentation branch and a token with read access to this repository, supplied through `GH_TOKEN`. Do not put the token in a command, document or committed file.

```sh
python3 scripts/export_project_archive.py
python3 scripts/validate_project_archive.py
git diff --stat -- docs/project-archive
git diff -- docs/project-archive/manifest.json
```

Review the changed documents and counts before committing them through a PR. The exporter reads issues and PR metadata; it does not merge PRs or change issue state. Retrieval spans multiple requests, so the result is not an atomic snapshot of GitHub.

Attachments remain links. Deleted content, edit histories, PR diffs, inline review threads and CI logs are outside this archive. Keep those with their source PR when they are needed for acceptance review.
