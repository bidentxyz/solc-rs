"""Resolve solc-bin binary names from compiler version strings.

Etherscan often strips leading zeros from the commit hash, so
`v0.5.14+commit.1f1aaa4` must match `solc-linux-amd64-v0.5.14+commit.01f1aaa4`.
"""

from __future__ import annotations


def long_version(name: str) -> str:
    """Return the `0.5.14+commit.01f1aaa4` part of a solc-bin file name."""
    prefix = "solc-linux-amd64-v"
    if name.startswith(prefix):
        return name[len(prefix) :]
    return name.lstrip("v")


def version_key(version: str) -> tuple[str, str | None]:
    """Return `(semver, commit)` with leading zeros stripped from commit."""
    version = version.lstrip("v")
    if "+commit." not in version:
        return version.split("+", 1)[0], None
    semver, commit = version.split("+commit.", 1)
    commit = commit.split(".")[0]
    return semver, commit.lstrip("0") or "0"


def resolve_solc_name(version: str, candidates: list[str]) -> str | None:
    """Return the unique solc-bin file name that matches `version`."""
    wanted = f"solc-linux-amd64-v{version.lstrip('v')}"
    if wanted in candidates:
        return wanted
    semver, commit = version_key(version)
    matches = []
    for name in candidates:
        name_semver, name_commit = version_key(long_version(name))
        if name_semver != semver:
            continue
        if commit is None:
            if "-nightly" not in name:
                matches.append(name)
            continue
        if name_commit == commit:
            matches.append(name)
    if len(matches) == 1:
        return matches[0]
    return None
