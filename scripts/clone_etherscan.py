#!/usr/bin/env python3
"""
Clone a verified contract from an Etherscan-compatible explorer into fixtures.

Fetches the verified source of a contract from the explorer API (using the
ETHERSCAN_API_KEY from the environment or the repository .env file), then
creates fixtures/solc-<major>.<minor>/<ContractName>.json as a standard-json
input and registers it in the fixture directory's compilers.json.

Usage:
    python3 scripts/clone_etherscan.py <explorer address URL>

Example:
    python3 scripts/clone_etherscan.py https://arbiscan.io/address/0xC31E54c7a869B9FcBEcc14363CF510d1c41fa443
    python3 scripts/clone_etherscan.py https://bscscan.com/token/0x8965349fb649a33a30cbfda057d8ec2c48abe2a2
"""

import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

from solc_bin import long_version, resolve_solc_name

# Explorer host -> (chain, chain id). The Etherscan API V2 endpoint serves
# every chain and requires the chainid parameter.
EXPLORERS: dict[str, tuple[str, str]] = {
    "arbiscan.io": ("arbitrum", "42161"),
    "etherscan.io": ("ethereum", "1"),
    "basescan.org": ("base", "8453"),
    "bscscan.com": ("bsc", "56"),
}

API_URL = "https://api.etherscan.io/v2/api"
ROOT = Path(__file__).resolve().parent.parent


def load_api_key(root: Path) -> str:
    """Return ETHERSCAN_API_KEY from the environment or root/.env."""
    api_key = os.environ.get("ETHERSCAN_API_KEY")
    if api_key:
        return api_key
    env_file = root / ".env"
    if env_file.is_file():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, _, value = line.partition("=")
            if key.strip() == "ETHERSCAN_API_KEY":
                return value.strip().strip('"').strip("'")
    print("Error: ETHERSCAN_API_KEY not set and not found in .env")
    sys.exit(1)


def parse_address(url: str) -> tuple[str, str, str]:
    """Return (chain, chain_id, address) for an explorer contract URL.

    Accepts both /address/<addr> and /token/<addr> pages; the token page
    points at the token's contract address.
    """
    parsed = urllib.parse.urlparse(url)
    host = parsed.netloc.lower()
    if host.startswith("www."):
        host = host[4:]
    explorer = EXPLORERS.get(host)
    if explorer is None:
        supported = ", ".join(EXPLORERS)
        print(f"Error: unsupported explorer host '{host}' (supported: {supported})")
        sys.exit(1)
    parts = [p for p in parsed.path.split("/") if p]
    if len(parts) < 2 or parts[0] not in ("address", "token"):
        print(
            "Error: expected a URL like "
            f"https://{host}/address/0x... or https://{host}/token/0x..."
        )
        sys.exit(1)
    address = parts[1]
    if not re.fullmatch(r"0x[0-9a-fA-F]{40}", address):
        print(f"Error: invalid address '{address}'")
        sys.exit(1)
    return explorer[0], explorer[1], address


def fetch_contract(chain_id: str, address: str, api_key: str) -> dict[str, Any]:
    """Return the verified contract source from the Etherscan API V2."""
    query = urllib.parse.urlencode(
        {
            "chainid": chain_id,
            "module": "contract",
            "action": "getsourcecode",
            "address": address,
            "apikey": api_key,
        }
    )
    url = f"{API_URL}?{query}"
    request = urllib.request.Request(url, headers={"User-Agent": "solc-rs"})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            data = json.load(response)
    except (urllib.error.URLError, json.JSONDecodeError) as e:
        print(f"Error: API request failed: {e}")
        sys.exit(1)
    result = data.get("result")
    if data.get("status") != "1" or not isinstance(result, list) or not result:
        detail = (
            result if isinstance(result, str) else data.get("message", "unknown error")
        )
        print(f"Error: API request failed: {detail}")
        sys.exit(1)
    return result[0]


def parse_source_wrapper(source: str) -> dict[str, Any] | None:
    """Parse an Etherscan SourceCode field, tolerating its escaping quirks.

    Multi-file contracts are stored as a standard-json input, either with
    just the outer pair of braces doubled or with every brace doubled.
    Older explorers store a direct file name -> content map without the
    standard-json wrapper. Returns None for plain Solidity sources.
    """
    candidates = [source]
    if source.startswith("{") and source.endswith("}"):
        candidates.append(source[1:-1])
    candidates.append(source.replace("{{", "{").replace("}}", "}"))
    for candidate in candidates:
        try:
            payload = json.loads(candidate)
        except json.JSONDecodeError:
            continue
        if isinstance(payload, dict) and "sources" in payload:
            return payload
        # Legacy multi-file format: a direct map of file names to source
        # entries, normalized into a standard-json input.
        if isinstance(payload, dict) and payload and is_source_map(payload):
            return {"language": "Solidity", "sources": payload, "settings": {}}
    return None


def is_source_map(payload: dict[str, Any]) -> bool:
    """Return whether payload maps file names to source entries."""
    for name, entry in payload.items():
        if name in ("language", "settings"):
            continue
        if isinstance(entry, str):
            continue
        if not isinstance(entry, dict) or "content" not in entry:
            return False
    return True


def fixture_dir_for(compiler_version: str) -> Path:
    """Return the fixtures directory for a compiler version, e.g. solc-0.4."""
    version = compiler_version.lstrip("v").split("+", 1)[0]
    major_minor = ".".join(version.split(".")[:2])
    return ROOT / "fixtures" / f"solc-{major_minor}"


def solc_binary_name(compiler_version: str) -> str:
    """Return the solc-bin file name for a compiler version."""
    return f"solc-linux-amd64-v{compiler_version.lstrip('v')}"


def main(argv: list[str]) -> int:
    if len(argv) != 1:
        print("Usage: python3 scripts/clone_etherscan.py <explorer contract URL>")
        sys.exit(1)

    api_key = load_api_key(ROOT)
    chain, chain_id, address = parse_address(argv[0])
    print(f"Fetching {chain} contract {address}")

    result = fetch_contract(chain_id, address, api_key)
    contract_name = result.get("ContractName") or "Contract"
    compiler_version = result.get("CompilerVersion") or ""
    if not compiler_version:
        print("Error: API response has no CompilerVersion")
        sys.exit(1)
    source = result.get("SourceCode") or ""
    if not source.strip():
        print("Error: contract has no source code (is it verified?)")
        sys.exit(1)
    local_names = [path.name for path in (ROOT / ".solc").glob("solc-linux-amd64-v*")]
    resolved = resolve_solc_name(compiler_version, local_names)
    if resolved is not None:
        compiler_version = long_version(resolved)
    else:
        compiler_version = compiler_version.lstrip("v")
    solc = solc_binary_name(compiler_version)

    fixture_dir = fixture_dir_for(compiler_version)
    wrapper = parse_source_wrapper(source)
    if wrapper is None:
        # Plain single-file sources are wrapped into a standard-json input
        wrapper = {
            "language": "Solidity",
            "sources": {f"{contract_name}.sol": {"content": source}},
            "settings": {},
        }
    # The compiler version is not part of the standard-json input format, so
    # it is stored as a custom top-level key that compile.py strips before
    # passing the input to solc.
    wrapper["version"] = compiler_version
    relative_file = f"{contract_name}.json"
    output = fixture_dir / relative_file
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(wrapper, indent=2) + "\n")
    print(f"Wrote {output}")

    binary = ROOT / ".solc" / solc
    if not binary.is_file():
        print(
            f"Note: {solc} is not downloaded yet, 'make fixtures' will download it",
            file=sys.stderr,
        )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
