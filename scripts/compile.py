#!/usr/bin/env python3
"""
Compile every standard-json input in fixtures/solc-*/*.json into out/.

Each input carries its compiler version in a custom top-level "version" key
(written by scripts/clone_etherscan.py), which is stripped before passing the
input to solc. Missing binaries are downloaded on demand from solc-bin with
sha256 verification and skipped when already present. The full solc output is
written to out/<file stem>/<file stem>.json inside the fixture directory.
"""

import hashlib
import json
import subprocess
import sys
import urllib.request
from pathlib import Path

from solc_bin import resolve_solc_name

ROOT = Path(__file__).resolve().parent.parent
BASE_URL = "https://binaries.soliditylang.org/linux-amd64"
DEFAULT_CONTRACT_SELECTION = [
    "abi",
    "evm.bytecode.object",
    "evm.deployedBytecode.object",
]


def _fetch(url: str) -> bytes:
    """Fetch a URL with a browser-like User-Agent (solc-bin blocks urllib's)."""
    request = urllib.request.Request(url, headers={"User-Agent": "solc-rs"})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def ensure_solc(version: str) -> Path:
    """Return the solc binary path for a version, downloading it if needed."""
    local_dir = ROOT / ".solc"
    local_dir.mkdir(parents=True, exist_ok=True)
    local_names = [path.name for path in local_dir.glob("solc-linux-amd64-v*")]
    name = resolve_solc_name(version, local_names)
    if name is not None:
        return local_dir / name

    list_json = json.loads(_fetch(f"{BASE_URL}/list.json"))
    builds = list_json.get("builds", [])
    remote_names = [build.get("path", "") for build in builds]
    name = resolve_solc_name(version, remote_names)
    if name is None:
        raise SystemExit(f"error: solc {version} not found in solc-bin list.json")
    sha256 = next(
        (build.get("sha256", "") for build in builds if build.get("path") == name),
        "",
    )
    if not sha256:
        raise SystemExit(f"error: {name} is missing a sha256 in list.json")
    binary = local_dir / name
    print(f"downloading {name}", file=sys.stderr)
    tmp = binary.with_suffix(".partial")
    try:
        tmp.write_bytes(_fetch(f"{BASE_URL}/{name}"))
        digest = hashlib.sha256(tmp.read_bytes()).hexdigest()
        if digest != sha256.removeprefix("0x"):
            raise SystemExit(f"error: sha256 mismatch for {name}")
        tmp.chmod(0o755)
        tmp.rename(binary)
    finally:
        tmp.unlink(missing_ok=True)
    return binary


def compile_input(solc_path: Path, input_file: Path, output_file: Path) -> bool:
    """Compile one standard-json input, always requesting the AST."""
    data = json.loads(input_file.read_text())
    data.pop("version", None)
    settings = data.setdefault("settings", {})
    selection = settings.setdefault("outputSelection", {})
    all_sources = selection.setdefault("*", {})
    all_sources.setdefault("", ["ast"])
    all_sources.setdefault("*", DEFAULT_CONTRACT_SELECTION)
    result = subprocess.run(
        [str(solc_path), "--standard-json"],
        input=json.dumps(data).encode(),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        print(
            f"error: solc failed for {input_file}: {result.stderr.decode()}",
            file=sys.stderr,
        )
        return False
    try:
        output = json.loads(result.stdout)
    except json.JSONDecodeError:
        print(f"error: solc returned invalid JSON for {input_file}", file=sys.stderr)
        return False
    errors = [e for e in output.get("errors", []) if e.get("severity") == "error"]
    if errors:
        for error in errors:
            message = error.get("formattedMessage") or error.get("message")
            print(f"error: {message}", file=sys.stderr)
        return False
    output_file.write_text(json.dumps(output, indent=2) + "\n")
    return True


def main() -> int:
    inputs = sorted(
        p for p in ROOT.glob("fixtures/solc-*/*.json") if p.name != "compilers.json"
    )
    if not inputs:
        print("error: no fixtures/solc-*/*.json inputs found", file=sys.stderr)
        return 1
    failed = 0
    for input_file in inputs:
        fixture_dir = input_file.parent
        data = json.loads(input_file.read_text())
        version = data.get("version", "")
        if not version:
            print(f"error: no version field in {input_file}", file=sys.stderr)
            return 1
        solc_path = ensure_solc(version)
        stem = input_file.stem
        output_file = fixture_dir / "out" / stem / f"{stem}.json"
        output_file.parent.mkdir(parents=True, exist_ok=True)
        print(f"compile {input_file.name} with {solc_path.name} -> out/{stem}")
        if not compile_input(solc_path, input_file, output_file):
            failed += 1
    if failed:
        print(f"error: {failed} compilation(s) failed", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
