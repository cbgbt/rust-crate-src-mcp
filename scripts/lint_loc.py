#!/usr/bin/env python3
"""Linter to detect Rust modules exceeding 500 lines of code."""

import sys
from pathlib import Path

MAX_LOC = 500

# Per-file overrides with justification (must be under this limit)
OVERRIDES = {
    # example: "src/big.rs": 625,
}


def count_lines(path: Path) -> int:
    return len(path.read_text().splitlines())


def is_test_file(path: Path) -> bool:
    return (
        "/tests/" in str(path)
        or path.name == "tests.rs"
        or path.name.endswith("_test.rs")
        or path.name.startswith("test_")
    )


def main():
    violations = []

    src_dir = Path("src")
    if src_dir.exists():
        for rs_file in src_dir.rglob("*.rs"):
            if is_test_file(rs_file):
                continue
            loc = count_lines(rs_file)
            rel_path = str(rs_file)
            limit = OVERRIDES.get(rel_path, MAX_LOC)
            if loc > limit:
                violations.append((rs_file, loc, limit))

    violations.sort(key=lambda x: -x[1])

    if violations:
        print("Modules exceeding LOC limits:\n")
        for path, loc, limit in violations:
            print(f"  {path}: {loc} lines (limit: {limit})")
        print(f"\nTotal: {len(violations)} module(s) need refactoring")
        return 1
    else:
        print("All modules are within LOC limits")
        return 0


if __name__ == "__main__":
    sys.exit(main())
