"""Validate archive integrity without network access or source-content execution."""
import collections
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import sys


def validate(root: Path) -> dict:
    manifest = json.loads((root / 'manifest.json').read_text(encoding='utf-8'))
    snapshot = json.loads((root / 'snapshot.json').read_text(encoding='utf-8'))
    for name, expected in manifest['files_sha256'].items():
        relative = PurePosixPath(name)
        if relative.is_absolute() or '..' in relative.parts:
            raise ValueError(f'Unsafe manifest path: {name}')
        path = root.joinpath(*relative.parts)
        if path.is_symlink() or not path.is_file():
            raise ValueError(f'Missing or unsafe archive file: {name}')
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f'Checksum mismatch: {name}')
    issues, prs = snapshot['issues'], snapshot['pull_requests']
    numbers = [item['number'] for item in issues]
    all_numbers = numbers + [item['number'] for item in prs]
    if len(all_numbers) != len(set(all_numbers)):
        raise ValueError('Duplicate issue/PR numbers')
    comment_ids = []
    for item in issues + prs:
        if hashlib.sha256((item.get('body') or '').encode()).hexdigest() != item['body_sha256']:
            raise ValueError('Changed source body')
        if len(item['discussion']) != item['comments']:
            raise ValueError('Missing conversation comments')
        for comment in item['discussion']:
            comment_ids.append(comment['id'])
            if hashlib.sha256(comment['body'].encode()).hexdigest() != comment['body_sha256']:
                raise ValueError('Changed source comment')
    if len(comment_ids) != len(set(comment_ids)):
        raise ValueError('Duplicate comment IDs')
    counts = {
        'issues': len(issues),
        'epics': sum(bool(re.match(r'^EPIC\s*:', item['title'], re.I)) for item in issues),
        'task_issues': sum('[TASK]' in item['title'].upper() for item in issues),
        'open_issues': sum(item['state'] == 'open' for item in issues),
        'closed_issues': sum(item['state'] == 'closed' for item in issues),
        'issue_comments': sum(len(item['discussion']) for item in issues),
        'pull_requests': len(prs),
        'pr_conversation_comments': sum(len(item['discussion']) for item in prs),
    }
    if counts != manifest['counts'] or counts != snapshot['counts']:
        raise ValueError('Summary counts do not match source snapshot')
    anchor = r'(?m)^<a id="issue-(\d+)"></a>$'
    combined = (root / 'ALL_ISSUES_AND_EPICS.md').read_text(encoding='utf-8')
    found = list(map(int, re.findall(anchor, combined)))
    if collections.Counter(found) != collections.Counter(numbers):
        raise ValueError('Combined document has missing or duplicate issues')
    if (root / 'ALL_ISSUES_AND_EPICS.txt').read_text(encoding='utf-8') != combined:
        raise ValueError('Text and Markdown combined documents differ')
    documented = []
    for document in manifest['documents']:
        found = list(map(int, re.findall(anchor, (root / document['path']).read_text(encoding='utf-8'))))
        if found != document['issue_numbers'] or len(found) != document['issue_count']:
            raise ValueError(f'Workstream membership mismatch: {document["path"]}')
        documented.extend(found)
    if collections.Counter(documented) != collections.Counter(numbers):
        raise ValueError('Workstream library does not contain every issue exactly once')
    return {'validation': 'passed', 'counts': counts, 'files_verified': len(manifest['files_sha256']),
            'retrieval_started_utc': manifest['retrieval_started_utc'],
            'retrieval_finished_utc': manifest['retrieval_finished_utc']}


if __name__ == '__main__':
    print(json.dumps(validate(Path(sys.argv[1] if len(sys.argv) > 1 else 'docs/project-archive')), indent=2))
