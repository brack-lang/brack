#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <new_version>"
  exit 1
fi

CURRENT_VERSION=$1

if [[ ! "$CURRENT_VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+-rc\.[0-9]+$ ]]; then
  echo "Invalid version format: $CURRENT_VERSION, expected: vX.Y.Z-rc.N"
  exit 1
fi

VERSION_BASE=$(echo $CURRENT_VERSION | sed -E 's/-rc\.[0-9]+$//')
RC_NUMBER=$(echo $CURRENT_VERSION | sed -E 's/^v[0-9]+\.[0-9]+\.[0-9]+-rc\.//')

if [ "$RC_NUMBER" -gt 1 ]; then
  PREV_VERSION="$VERSION_BASE-rc.$((RC_NUMBER - 1))"
else
  PREV_VERSION=$(gh release list | grep -v "rc" | awk '{print $1}' | sort -V | awk -v current_version="$VERSION_BASE" '$1 < current_version' | tail -n 1)
fi

if [ -z "$PREV_VERSION" ]; then
  echo "No previous stable release found." >&2
  exit 1
fi

PREV_DATE=$(gh release view "$PREV_VERSION" --json publishedAt --jq '.publishedAt' 2>/dev/null)

if [ -z "$PREV_DATE" ]; then
  echo "Previous version $PREV_VERSION not found." >&2
  exit 1
fi

echo "Comparing PRs merged between $PREV_VERSION and $CURRENT_VERSION..." >&2

PR_LIST=$(gh pr list --state merged --base develop --search "merged:>$PREV_DATE" --json number,title,mergedAt,author --jq 'sort_by(.number)[] | "* \(.title) by @\(.author.login) in https://github.com/brack-lang/brack/pull/\(.number)"')

if [ -z "$PR_LIST" ]; then
  echo "No PRs were merged between $PREV_VERSION and $CURRENT_VERSION." >&2
else
  echo "PRs merged between $PREV_VERSION and $CURRENT_VERSION:" >&2
  echo "## What's Changed\n$PR_LIST"
fi
