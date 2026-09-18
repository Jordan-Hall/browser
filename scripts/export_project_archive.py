#!/usr/bin/env python3
"""Export every repository issue and conversation into project documents."""
from __future__ import annotations
import collections
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import time
import urllib.error
import urllib.request

REPO = "Jordan-Hall/browser"
BASE = f"https://api.github.com/repos/{REPO}/"
OUT = Path("docs/project-archive")
FIELDS = ("number", "title", "state", "state_reason", "body", "html_url", "created_at", "updated_at", "closed_at", "locked", "comments", "parent_issue_url", "sub_issues_summary", "issue_dependencies_summary")

def now():
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds")

def api(url):
    if not (url.lower().startswith(BASE.lower()) or url.startswith("https://api.github.com/repositories/1371315992/")):
        raise ValueError("Request must stay inside the selected repository")
    headers = {"Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28", "User-Agent": "browser-project-document-export"}
    if os.environ.get("GH_TOKEN"):
        headers["Authorization"] = "Bearer " + os.environ["GH_TOKEN"]
    for attempt in range(4):
        try:
            with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=60) as response:
                data = response.read(32 * 1024 * 1024 + 1)
                if len(data) > 32 * 1024 * 1024:
                    raise ValueError("API page exceeds export budget")
                return json.loads(data), response.headers
        except urllib.error.HTTPError as error:
            if attempt == 3 or error.code not in (429, 500, 502, 503, 504):
                raise
            time.sleep(min(30, int(error.headers.get("Retry-After", 2 ** attempt))))
    raise RuntimeError("Request retries exhausted")

def pages(path):
    url, results = BASE + path, []
    for _ in range(500):
        page, headers = api(url)
        if not isinstance(page, list):
            raise ValueError("Expected list page")
        results.extend(page)
        print(f"Read {len(page)} records; accumulated {len(results)}", flush=True)
        links = re.findall(r'<([^>]+)>;\s*rel="next"', headers.get("Link", ""))
        if not links:
            return results
        url = links[0]
    raise ValueError("Pagination budget exceeded")

def number(url):
    match = re.search(r"/(?:issues|pulls|pull)/(\d+)(?:$|/|#)", url or "")
    return int(match[1]) if match else None

def digest(text):
    return hashlib.sha256((text or "").encode()).hexdigest()

def normalize(item):
    result = {name: item.get(name) for name in FIELDS}
    result["author"] = (item.get("user") or {}).get("login")
    result["labels"] = [x["name"] for x in item.get("labels", [])]
    result["assignees"] = [x["login"] for x in item.get("assignees", [])]
    milestone = item.get("milestone")
    result["milestone"] = None if not milestone else {key: milestone.get(key) for key in ("number", "title", "state", "due_on")}
    result["kind"] = "pull_request" if "pull_request" in item else "issue"
    result["native_parent"] = number(item.get("parent_issue_url"))
    match = re.search(r"(?im)^\s*(?:Parent|Programme):\s*#(\d+)\b", item.get("body") or "")
    result["declared_parent"] = int(match[1]) if match else None
    result["body_sha256"] = digest(item.get("body"))
    result["discussion"] = []
    return result

def normalize_comment(comment):
    return {"id": comment["id"], "body": comment.get("body") or "", "author": (comment.get("user") or {}).get("login", "unknown"), "created_at": comment["created_at"], "updated_at": comment["updated_at"], "html_url": comment["html_url"], "body_sha256": digest(comment.get("body"))}

def group_issues(issues):
    by_id = {item["number"]: item for item in issues}
    epics = {n: item for n, item in by_id.items() if re.match(r"^EPIC\s*:", item["title"], re.I)}
    child_to_epic = {}
    for n, epic in epics.items():
        section = re.search(r"(?ims)^##\s+Child issues\s*\n(.*?)(?=^##\s|\Z)", epic.get("body") or "")
        for child in re.findall(r"(?m)^\s*[-*]\s+\[[ xX]\]\s+#(\d+)\b", section[1] if section else ""):
            child_to_epic[int(child)] = n
    groups, warnings = collections.defaultdict(list), []
    for item in issues:
        n, native, declared = item["number"], item["native_parent"], item["declared_parent"]
        if native and declared and native != declared:
            warnings.append(f"Issue #{n}: native parent #{native}; body-declared parent #{declared}. Both preserved.")
        current, seen, group = n, set(), 0
        while current in by_id and current not in seen:
            seen.add(current)
            if current in epics:
                group = current
                break
            node = by_id[current]
            if node["native_parent"]:
                current = node["native_parent"]
                continue
            if current in child_to_epic:
                group = child_to_epic[current]
                break
            if not node["declared_parent"]:
                break
            current = node["declared_parent"]
        item["document_epic"] = group or None
        item["referenced_numbers"] = sorted({int(x) for x in re.findall(r"(?<![A-Za-z0-9])#(\d+)\b", item.get("body") or "")})
        groups[group].append(item)
    for values in groups.values():
        values.sort(key=lambda item: (0 if item["number"] in epics else 1, item["number"]))
    return dict(groups), warnings

def cell(value):
    return str(value or "").replace("|", "\\|").replace("\n", " ")

def embed(body, level):
    lines, fence = [], None
    for line in body.splitlines():
        start = re.match(r"^\s*(`{3,}|~{3,})", line)
        if start:
            mark = start[1][0]
            fence = mark if fence is None else None if mark == fence else fence
            lines.append(line)
        elif fence is None and re.match(r"^#{1,6}\s", line):
            match = re.match(r"^(#{1,6})(\s.*)$", line)
            lines.append("#" * min(6, len(match[1]) + level) + match[2])
        else:
            lines.append(line)
    return "\n".join(lines)

def entry(item, children, mentions):
    n = item["number"]
    bits = [f'<a id="issue-{n}"></a>', f'## #{n} — {item["title"]}', "", f'**GitHub state:** {item["state"]} | **State reason:** {item.get("state_reason") or "not supplied"}', f'**Source:** {item["html_url"]}', f'**Created:** {item["created_at"]} | **Updated:** {item["updated_at"]}', f'**Author:** {item.get("author") or "unknown"} | **Assignees:** {", ".join(item["assignees"]) or "none"}', f'**Labels:** {", ".join(item["labels"]) or "none"} | **Milestone:** {(item.get("milestone") or {}).get("title", "none")}', f'**Native parent:** {"#" + str(item["native_parent"]) if item["native_parent"] else "none recorded"} | **Body-declared parent:** {"#" + str(item["declared_parent"]) if item["declared_parent"] else "none recorded"}']
    if children.get(n):
        bits.append("**Native child issues:** " + ", ".join(f"#{x}" for x in children[n]))
    if mentions.get(n):
        bits.append("**PRs mentioning this issue:** " + ", ".join(f"[#{x}](https://github.com/{REPO}/pull/{x})" for x in mentions[n]) + " (text references, not verified implementation or acceptance)")
    if item.get("native_dependencies"):
        bits.append("**Native dependencies:** " + json.dumps(item["native_dependencies"], ensure_ascii=False))
    bits.extend(["", "### Original description", "", embed(item.get("body") or "[No description supplied.]", 2), "", f'### Discussion ({len(item["discussion"])} comments)', ""])
    for comment in item["discussion"]:
        bits.extend([f'#### Comment {comment["id"]} — {comment["author"]} — {comment["created_at"]}', "", f'Source: {comment["html_url"]} | Updated: {comment["updated_at"]}', "", embed(comment["body"], 4), ""])
    if not item["discussion"]:
        bits.append("No issue-conversation comments were returned at export time.")
    return "\n".join(bits) + "\n\n---\n\n"

def write_documents(snapshot):
    issues, prs = snapshot["issues"], snapshot["pull_requests"]
    groups, warnings = group_issues(issues)
    snapshot["relationship_notes"] = warnings
    children, mentions = collections.defaultdict(list), collections.defaultdict(list)
    for issue in issues:
        if issue["native_parent"]:
            children[issue["native_parent"]].append(issue["number"])
    ids = {item["number"] for item in issues}
    for pr in prs:
        text = pr["title"] + "\n" + (pr.get("body") or "")
        for n in sorted({int(x) for x in re.findall(r"(?<![A-Za-z0-9])#(\d+)\b", text)} & ids):
            mentions[n].append(pr["number"])
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "workstreams").mkdir(exist_ok=True)
    scope = (f'Repository: {REPO}\n\nRetrieved from {snapshot["retrieval_started_utc"]} to {snapshot["retrieval_finished_utc"]}. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.\n\n')
    manifest, assembled = [], ["# Intent Browser — all issues and epics\n\n", scope]
    by_id = {item["number"]: item for item in issues}
    for group, values in sorted(groups.items()):
        title = by_id[group]["title"].removeprefix("EPIC: ") if group else "Programme and ungrouped issues"
        slug = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")[:80]
        filename = f"{group:03d}-{slug}.md"
        toc = "\n".join(f'- [#{item["number"]} — {item["title"]}](#issue-{item["number"]})' for item in values)
        content = f"# {title}\n\n" + scope + f"**Issues in this document:** {len(values)}\n\n## Contents\n\n" + toc + "\n\n---\n\n"
        content += "".join(entry(item, children, mentions) for item in values)
        (OUT / "workstreams" / filename).write_text(content, encoding="utf-8")
        assembled.append(content)
        manifest.append({"epic_issue": group or None, "title": title, "path": f"workstreams/{filename}", "issue_count": len(values), "comment_count": sum(len(item["discussion"]) for item in values), "issue_numbers": [item["number"] for item in values]})
    snapshot["documents"] = manifest
    snapshot["counts"] = {"issues": len(issues), "epics": sum(bool(item["epic_issue"]) for item in manifest), "task_issues": sum("[TASK]" in item["title"].upper() for item in issues), "open_issues": sum(item["state"] == "open" for item in issues), "closed_issues": sum(item["state"] == "closed" for item in issues), "issue_comments": sum(len(item["discussion"]) for item in issues), "pull_requests": len(prs), "pr_conversation_comments": sum(len(item["discussion"]) for item in prs)}
    combined = "\n\n".join(assembled)
    for name in ("ALL_ISSUES_AND_EPICS.md", "ALL_ISSUES_AND_EPICS.txt"):
        (OUT / name).write_text(combined, encoding="utf-8")
    pr_doc = "# Pull-request context register\n\n" + scope + "PR titles, descriptions, branch/commit metadata and conversation comments only; not a code review or CI verification.\n\n"
    for pr in prs:
        pr_doc += entry(pr, {}, {}) + "### PR metadata\n\n```json\n" + json.dumps(pr.get("pr_metadata", {}), ensure_ascii=False, indent=2) + "\n```\n\n"
    (OUT / "PULL_REQUEST_REGISTER.md").write_text(pr_doc, encoding="utf-8")
    doc_for = {n: item["path"] for item in manifest for n in item["issue_numbers"]}
    rows = ["# Complete issue register\n", scope, "| Issue | Title | Kind | GitHub state | Native parent | Declared parent | Document |", "|---|---|---|---|---|---|---|"]
    for item in issues:
        kind = "Epic" if item["title"].startswith("EPIC:") else "Task" if "[TASK]" in item["title"] else "Programme" if item["number"] == 1 else "Requirement / issue"
        n = item["number"]
        rows.append(f'| [#{n}]({item["html_url"]}) | {cell(item["title"])} | {kind} | {item["state"]} | {item["native_parent"] or "—"} | {item["declared_parent"] or "—"} | [Open]({doc_for[n]}#issue-{n}) |')
    (OUT / "ISSUE_REGISTER.md").write_text("\n".join(rows) + "\n", encoding="utf-8")
    readme = ["# Intent Browser — project document library\n", scope, "## Start here\n", "For one project upload, use ALL_ISSUES_AND_EPICS.txt or its Markdown equivalent. For focused context, use the individual workstreams/ documents instead. The combined and workstream files contain the same issues; uploading both creates duplicate context.\n", "ISSUE_REGISTER.md indexes every issue. PULL_REQUEST_REGISTER.md supplies separate PR context. snapshot.json retains exact body/comment strings and source metadata. manifest.json records counts, retrieval boundaries, SHA-256 checksums and export checks.\n", "## Verified export counts\n", "```json", json.dumps(snapshot["counts"], indent=2), "```\n", "## Workstream documents\n", "| Document | Epic | Issues | Comments |", "|---|---|---:|---:|"]
    for item in manifest:
        readme.append(f'| [{cell(item["title"])}]({item["path"]}) | {"#" + str(item["epic_issue"]) if item["epic_issue"] else "Programme / ungrouped"} | {item["issue_count"]} | {item["comment_count"]} |')
    readme.extend(["\n## Relationship notes\n", "Native parent links take precedence for grouping; explicit epic child lists and body-declared parents provide fallbacks. Mismatches are retained, not rewritten.\n"])
    readme.extend(f"- {warning}" for warning in warnings)
    readme.extend(["\n## Completeness boundary\n", "All issue records returned by state=all pagination are included exactly once in the workstream library. Pull requests are separated rather than miscounted as issues. Conversation comment counts are reconciled per record. Concurrent edits after a record was read are not captured. No issue status, task checkbox, label or runtime source is changed by this export.\n"])
    (OUT / "README.md").write_text("\n".join(readme) + "\n", encoding="utf-8")
    (OUT / "snapshot.json").write_text(json.dumps(snapshot, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    flattened = [n for item in manifest for n in item["issue_numbers"]]
    if len(flattened) != len(set(flattened)) or set(flattened) != ids:
        raise ValueError("Issue coverage is incomplete or duplicated")
    for item in issues + prs:
        if len(item["discussion"]) != item["comments"] or item["body_sha256"] != digest(item.get("body")):
            raise ValueError(f'Unreconciled record #{item["number"]}')
        for comment in item["discussion"]:
            if comment["body_sha256"] != digest(comment["body"]):
                raise ValueError("Comment digest changed")
    checks = {str(path.relative_to(OUT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(OUT.rglob("*")) if path.is_file() and path.name != "manifest.json"}
    metadata = {"schema_version": 1, "repository": REPO, "retrieval_started_utc": snapshot["retrieval_started_utc"], "retrieval_finished_utc": snapshot["retrieval_finished_utc"], "counts": snapshot["counts"], "all_returned_issues_included_once": True, "comment_counts_reconciled": True, "exact_body_and_comment_strings_in_snapshot": True, "files_sha256": checks, "documents": manifest}
    (OUT / "manifest.json").write_text(json.dumps(metadata, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

def main():
    started = now()
    raw = pages("issues?state=all&sort=created&direction=asc&per_page=100")
    records = {item["number"]: normalize(item) for item in raw}
    if len(records) != len(raw):
        raise ValueError("Duplicate paginated issues; retry against quiet repository")
    seen = set()
    for comment in pages("issues/comments?sort=created&direction=asc&per_page=100"):
        n = number(comment["issue_url"])
        if n not in records:
            continue
        if comment["id"] in seen:
            raise ValueError("Duplicate paginated comment")
        seen.add(comment["id"])
        records[n]["discussion"].append(normalize_comment(comment))
    for n, item in list(records.items()):
        if len(item["discussion"]) != item["comments"]:
            refreshed, _ = api(BASE + f"issues/{n}")
            records[n] = normalize(refreshed)
            records[n]["discussion"] = [normalize_comment(c) for c in pages(f"issues/{n}/comments?per_page=100")]
        records[n]["discussion"].sort(key=lambda c: (c["created_at"], c["id"]))
    for pr in pages("pulls?state=all&sort=created&direction=asc&per_page=100"):
        if pr["number"] in records:
            records[pr["number"]]["pr_metadata"] = {"draft": pr["draft"], "merged_at": pr["merged_at"], "base_branch": pr["base"]["ref"], "base_sha": pr["base"]["sha"], "head_branch": pr["head"]["ref"], "head_sha": pr["head"]["sha"], "merge_commit_sha": pr.get("merge_commit_sha")}
    for item in records.values():
        summary = item.get("issue_dependencies_summary") or {}
        for kind, field in (("blocked_by", "total_blocked_by"), ("blocking", "total_blocking")):
            if summary.get(field, 0):
                item.setdefault("native_dependencies", {})[kind] = [x["number"] for x in pages(f'issues/{item["number"]}/dependencies/{kind}?per_page=100')]
    ordered = [records[n] for n in sorted(records)]
    snapshot = {"schema_version": 1, "repository": REPO, "retrieval_started_utc": started, "retrieval_finished_utc": now(), "issues": [x for x in ordered if x["kind"] == "issue"], "pull_requests": [x for x in ordered if x["kind"] == "pull_request"]}
    write_documents(snapshot)
    print(json.dumps(snapshot["counts"], indent=2))

if __name__ == "__main__":
    main()
