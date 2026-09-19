#!/usr/bin/env python3
"""
archive_source.py
=================
Universal source code archiver. Run from any project root to create a clean
zip of your source code, automatically skipping build artifacts, dependencies,
and other unnecessary files/folders.

Usage:
    python archive_source.py                  # Archive current directory
    python archive_source.py -o myproject.zip # Custom output name
    python archive_source.py --dry-run        # Preview what would be included
    python archive_source.py --verbose        # Show every file being added
    python archive_source.py --list-skipped   # Show what's being skipped
"""

import os
import sys
import zipfile
import argparse
import fnmatch
from datetime import datetime
from pathlib import Path

# ─────────────────────────────────────────────────────────────────────────────
# SKIP RULES — Add/remove patterns as needed
# ─────────────────────────────────────────────────────────────────────────────

SKIP_DIRS = {
    # JavaScript / Node
    "node_modules", ".npm", ".pnpm-store", ".yarn", ".pnp",
    "bower_components",

    # Rust / Cargo
    "target",

    # Python
    "__pycache__", ".venv", "venv", "env", ".env", ".tox",
    ".mypy_cache", ".pytest_cache", ".ruff_cache",
    "*.egg-info", "dist", "build", "eggs", ".eggs",

    # Java / Kotlin / Android
    ".gradle", "build", ".idea", "out",

    # .NET / C#
    "bin", "obj", "packages",

    # Go
    "vendor",

    # PHP
    "vendor",

    # Ruby
    ".bundle",

    # Dart / Flutter
    ".dart_tool", ".pub-cache",

    # Version control
    ".git", ".svn", ".hg", ".bzr",

    # IDE / Editor
    ".vs", ".vscode", ".idea", ".fleet",
    ".eclipse", ".settings", ".project",

    # OS junk
    ".DS_Store", "__MACOSX", "Thumbs.db",
    "$RECYCLE.BIN", "System Volume Information",

    # CI / Docker / Infra
    ".docker", ".terraform", ".serverless",

    # Misc build/cache
    ".cache", ".parcel-cache", ".next", ".nuxt", ".svelte-kit",
    ".turbo", ".vercel", ".output", ".netlify",
    "coverage", ".nyc_output", "htmlcov",
    ".angular", ".expo",

    # This script's own output
    ".gemini",
}

SKIP_FILES = {
    # Lock files (large, regenerable)
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "Cargo.lock",
    "Gemfile.lock",
    "poetry.lock",
    "composer.lock",
    "Pipfile.lock",

    # OS files
    ".DS_Store",
    "Thumbs.db",
    "desktop.ini",

    # Editor swap/backup
    "*.swp", "*.swo", "*.swn",
    "*~",
    "*.bak",
    "*.tmp",
}

SKIP_EXTENSIONS = {
    # Compiled / binary
    ".exe", ".dll", ".so", ".dylib", ".o", ".obj", ".a", ".lib",
    ".class", ".jar", ".war", ".ear",
    ".pyc", ".pyo", ".pyd",
    ".wasm",

    # Archives (don't nest archives)
    ".zip", ".tar", ".gz", ".bz2", ".xz", ".7z", ".rar",

    # Large media (optional — remove if you want to include these)
    # ".mp4", ".avi", ".mov", ".mkv",
    # ".mp3", ".wav", ".flac",

    # Database files
    # ".db-shm", ".db-wal",

    # Installer outputs
    ".msi", ".nsis", ".deb", ".rpm", ".dmg", ".AppImage",
}

# Max file size to include (default: 50 MB)
MAX_FILE_SIZE_MB = 150


# ─────────────────────────────────────────────────────────────────────────────
# Logic
# ─────────────────────────────────────────────────────────────────────────────

def should_skip_dir(dirname: str) -> bool:
    """Check if a directory name matches any skip pattern."""
    lower = dirname.lower()
    for pattern in SKIP_DIRS:
        if fnmatch.fnmatch(lower, pattern.lower()):
            return True
    return False


def should_skip_file(filename: str, filepath: str) -> bool:
    """Check if a file should be skipped based on name, extension, or size."""
    lower = filename.lower()

    # Check exact name matches and glob patterns
    for pattern in SKIP_FILES:
        if fnmatch.fnmatch(lower, pattern.lower()):
            return True

    # Check extension
    _, ext = os.path.splitext(lower)
    if ext in SKIP_EXTENSIONS:
        return True

    # Check file size
    try:
        size_mb = os.path.getsize(filepath) / (1024 * 1024)
        if size_mb > MAX_FILE_SIZE_MB:
            return True
    except OSError:
        pass

    return False


def collect_files(root_dir: str, list_skipped: bool = False):
    """Walk the directory tree and collect files to archive."""
    files_to_add = []
    skipped_dirs = []
    skipped_files = []
    total_size = 0

    for dirpath, dirnames, filenames in os.walk(root_dir):
        # Filter out skipped directories (modifying dirnames in-place)
        original_dirs = dirnames.copy()
        dirnames[:] = [d for d in dirnames if not should_skip_dir(d)]

        if list_skipped:
            for d in original_dirs:
                if d not in dirnames:
                    skipped_dirs.append(os.path.join(dirpath, d))

        for filename in filenames:
            filepath = os.path.join(dirpath, filename)
            relpath = os.path.relpath(filepath, root_dir)

            # Skip the archive output itself
            if filename == os.path.basename(sys.argv[0]):
                continue

            if should_skip_file(filename, filepath):
                if list_skipped:
                    skipped_files.append(relpath)
                continue

            try:
                size = os.path.getsize(filepath)
            except OSError:
                continue

            files_to_add.append((filepath, relpath, size))
            total_size += size

    return files_to_add, skipped_dirs, skipped_files, total_size


def format_size(size_bytes: int) -> str:
    """Format bytes into human-readable size."""
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size_bytes < 1024:
            return f"{size_bytes:.1f} {unit}"
        size_bytes /= 1024
    return f"{size_bytes:.1f} TB"


def main():
    parser = argparse.ArgumentParser(
        description="Archive source code, skipping build artifacts and dependencies.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python archive_source.py                  # Archive with auto-generated name
  python archive_source.py -o src.zip       # Custom output name
  python archive_source.py --dry-run        # Preview without creating zip
  python archive_source.py --list-skipped   # Show skipped files/dirs
  python archive_source.py --max-size 100   # Allow files up to 100 MB
        """,
    )
    parser.add_argument('-o', '--output', help='Output zip filename')
    parser.add_argument('-d', '--directory', default='.', help='Root directory to archive (default: current)')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be archived without creating the zip')
    parser.add_argument('--verbose', action='store_true', help='Print every file being added')
    parser.add_argument('--list-skipped', action='store_true', help='Print skipped files and directories')
    global MAX_FILE_SIZE_MB
    parser.add_argument('--max-size', type=int, default=MAX_FILE_SIZE_MB, help=f'Max file size in MB (default: {MAX_FILE_SIZE_MB})')
    args = parser.parse_args()

    MAX_FILE_SIZE_MB = args.max_size

    root_dir = os.path.abspath(args.directory)
    project_name = os.path.basename(root_dir)

    if not os.path.isdir(root_dir):
        print(f"Error: '{root_dir}' is not a directory.")
        sys.exit(1)

    # Generate output filename
    if args.output:
        zip_name = args.output
    else:
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        zip_name = f"{project_name}_source_{timestamp}.zip"

    zip_path = os.path.join(root_dir, zip_name)

    print(f"╔══════════════════════════════════════════════════════════════╗")
    print(f"║  📦 Source Archiver                                         ║")
    print(f"╠══════════════════════════════════════════════════════════════╣")
    print(f"║  Project : {project_name:<49s}║")
    print(f"║  Root    : {root_dir:<49s}║")
    if not args.dry_run:
        print(f"║  Output  : {zip_name:<49s}║")
    print(f"╚══════════════════════════════════════════════════════════════╝")
    print()

    # Collect files
    print("  Scanning files...")
    files, skipped_dirs, skipped_files, total_size = collect_files(
        root_dir, list_skipped=args.list_skipped
    )

    # Show skipped items
    if args.list_skipped:
        if skipped_dirs:
            print(f"\n  ── Skipped directories ({len(skipped_dirs)}) ──")
            for d in sorted(skipped_dirs)[:30]:
                print(f"    ✗ {os.path.relpath(d, root_dir)}/")
            if len(skipped_dirs) > 30:
                print(f"    ... and {len(skipped_dirs) - 30} more")

        if skipped_files:
            print(f"\n  ── Skipped files ({len(skipped_files)}) ──")
            for f in sorted(skipped_files)[:30]:
                print(f"    ✗ {f}")
            if len(skipped_files) > 30:
                print(f"    ... and {len(skipped_files) - 30} more")
        print()

    # Summary
    print(f"  Found {len(files):,} files ({format_size(total_size)})")

    if args.dry_run:
        print(f"\n  ── Files to include ──")
        for filepath, relpath, size in sorted(files, key=lambda x: x[1]):
            print(f"    {relpath} ({format_size(size)})")
        print(f"\n  DRY RUN — no zip created.")
        return

    if not files:
        print("  No files to archive!")
        sys.exit(1)

    # Create zip
    print(f"  Creating archive...")
    added = 0
    with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED, compresslevel=6) as zf:
        for filepath, relpath, size in files:
            # Don't include the zip file itself
            if os.path.abspath(filepath) == os.path.abspath(zip_path):
                continue
            try:
                zf.write(filepath, relpath)
                added += 1
                if args.verbose:
                    print(f"    + {relpath} ({format_size(size)})")
            except (PermissionError, OSError) as e:
                print(f"    ⚠ Skipped (error): {relpath} — {e}")

    zip_size = os.path.getsize(zip_path)
    ratio = (1 - zip_size / total_size) * 100 if total_size > 0 else 0

    print()
    print(f"  ✅ Done!")
    print(f"  ── Archive: {zip_name}")
    print(f"  ── Files  : {added:,}")
    print(f"  ── Size   : {format_size(zip_size)} (compressed {ratio:.0f}%)")
    print(f"  ── Path   : {zip_path}")


if __name__ == '__main__':
    main()
