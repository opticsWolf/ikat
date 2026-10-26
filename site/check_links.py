"""Check the site's ABSOLUTE links (the local refs/anchors/CSS assets
are already gated by the inline validator in pages.yml — this closes
the remaining gap: absolute hrefs rot silently when repo paths move,
e.g. skills/ -> python/ikat/skills/ in M7.9).

Usage (any python 3.10+, stdlib only):
    python site/check_links.py [path/to/index.html]

Every `href`/`src` with a scheme is GETted once (two tries on
transient errors); 200 passes, anything else fails the run with the
offending URL and status. GitHub blob links return the HTML page —
200 is the verdict; a 404 names the moved path.
"""

import re
import sys
import time
import urllib.request
from html.parser import HTMLParser
from pathlib import Path


class Refs(HTMLParser):
    def __init__(self):
        super().__init__()
        self.absolute = []

    def handle_starttag(self, tag, attrs):
        for key in ("href", "src"):
            v = dict(attrs).get(key, "")
            if "://" in v:
                self.absolute.append(v)


def check(html_path: Path) -> int:
    refs = Refs()
    refs.feed(html_path.read_text(encoding="utf-8"))
    failures = []
    seen = set()
    for url in refs.absolute:
        if url in seen:
            continue
        seen.add(url)
        status = None
        for attempt in (1, 2):
            try:
                req = urllib.request.Request(
                    url, headers={"User-Agent": "ikat-link-check"})
                with urllib.request.urlopen(req, timeout=30) as r:
                    status = r.status
                    r.read()
                break
            except urllib.error.HTTPError as e:
                status = e.code
                if e.code not in (429, 502, 503):
                    break
            except Exception as e:
                status = str(e)
                if attempt == 1:
                    time.sleep(2)
        ok = status == 200
        print(f"{'OK ' if ok else 'FAIL'} {status} {url}")
        if not ok:
            failures.append(url)
    print(f"{len(seen)} absolute links, {len(failures)} failed")
    return 1 if failures else 0


if __name__ == "__main__":
    path = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parent / "index.html")
    raise SystemExit(check(path))
