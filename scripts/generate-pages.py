#!/usr/bin/env python3
"""Generate a static HTML page from OOGA-BOOGA.md for GitHub Pages."""

import re
import sys
from pathlib import Path


def parse_md_tables(md_text: str):
    """Extract markdown tables from text; return list of (title, headers, rows)."""
    tables = []
    lines = md_text.splitlines()
    i = 0
    while i < len(lines):
        # Look for table header line (contains |)
        if "|" in lines[i] and "---" in lines[i + 1] if i + 1 < len(lines) else False:
            header_line = lines[i]
            headers = [h.strip() for h in header_line.split("|") if h.strip()]
            i += 2  # skip separator
            rows = []
            while i < len(lines) and "|" in lines[i]:
                row = [c.strip() for c in lines[i].split("|") if c.strip()]
                if row:
                    rows.append(row)
                i += 1
            # Try to find a title (nearest heading above)
            title = ""
            for j in range(i - len(rows) - 3, -1, -1):
                if lines[j].startswith("#"):
                    title = lines[j].lstrip("# ").strip()
                    break
            tables.append((title, headers, rows))
        else:
            i += 1
    return tables


def md_to_html(md_text: str) -> str:
    """Convert OOGA-BOOGA.md into a simple HTML page."""
    tables = parse_md_tables(md_text)

    sections = []
    for title, headers, rows in tables:
        if not rows:
            continue
        th = "".join(f"<th>{h}</th>" for h in headers)
        trs = ""
        for row in rows:
            tds = ""
            for cell in row:
                # Replace emoji/status markers with styled spans
                cell_html = (
                    cell.replace("✅", '<span class="status ok">✅</span>')
                    .replace("📝", '<span class="status planned">📝</span>')
                    .replace("❌", '<span class="status omit">❌</span>')
                )
                # Code formatting
                cell_html = re.sub(r"`([^`]+)`", r"<code>\1</code>", cell_html)
                tds += f"<td>{cell_html}</td>"
            trs += f"<tr>{tds}</tr>"
        sections.append(
            f'<section><h2>{title}</h2>'
            f'<table><thead><tr>{th}</tr></thead>'
            f'<tbody>{trs}</tbody></table></section>'
        )

    # Also include the markdown body as HTML-ish content
    body_paras = []
    in_table = False
    for line in md_text.splitlines():
        if "|" in line and "---" in line:
            in_table = True
            continue
        if in_table and "|" not in line:
            in_table = False
        if in_table:
            continue
        if line.startswith("#"):
            level = len(line) - len(line.lstrip("#"))
            text = line.lstrip("# ").strip()
            body_paras.append(f"<h{level}>{text}</h{level}>")
        elif line.strip():
            body_paras.append(f"<p>{line}</p>")

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Ooga-Booga-Fuzz Coverage Matrix</title>
<style>
  :root {{
    --bg: #0d1117;
    --fg: #c9d1d9;
    --accent: #58a6ff;
    --table-bg: #161b22;
    --table-border: #30363d;
    --ok: #3fb950;
    --planned: #d29922;
    --omit: #8b949e;
  }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    background: var(--bg);
    color: var(--fg);
    line-height: 1.6;
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }}
  h1 {{ color: var(--accent); border-bottom: 1px solid var(--table-border); padding-bottom: .5rem; }}
  h2 {{ color: var(--accent); margin-top: 2rem; }}
  table {{ width: 100%; border-collapse: collapse; margin: 1rem 0; background: var(--table-bg); }}
  th, td {{ padding: .6rem .8rem; border: 1px solid var(--table-border); text-align: left; }}
  th {{ background: #21262d; font-weight: 600; }}
  tr:hover {{ background: #1f242c; }}
  code {{ background: #21262d; padding: .15rem .4rem; border-radius: 4px; font-size: .9em; }}
  .status {{ font-weight: 700; }}
  .ok {{ color: var(--ok); }}
  .planned {{ color: var(--planned); }}
  .omit {{ color: var(--omit); }}
  footer {{ margin-top: 3rem; font-size: .85rem; color: #8b949e; border-top: 1px solid var(--table-border); padding-top: 1rem; }}
</style>
</head>
<body>
<h1>Ooga-Booga-Fuzz: Entropylab Coverage Matrix</h1>
<p>Generated from <a href="https://github.com/bitcoincore-dev/ooga-booga-fuzz/blob/v2/OOGA-BOOGA.md">OOGA-BOOGA.md</a>.</p>
{"".join(sections)}
<footer>
  <p>Last updated: {__import__('datetime').datetime.utcnow().strftime('%Y-%m-%d %H:%M UTC')}</p>
</footer>
</body>
</html>
"""
    return html


def main():
    ooga_path = Path(__file__).parent.parent / "OOGA-BOOGA.md"
    out_path = Path("site/index.html")
    out_path.parent.mkdir(parents=True, exist_ok=True)
    if not ooga_path.exists():
        print(f"Error: {ooga_path} not found", file=sys.stderr)
        sys.exit(1)
    html = md_to_html(ooga_path.read_text())
    out_path.write_text(html)
    print(f"Generated {out_path}")


if __name__ == "__main__":
    main()
