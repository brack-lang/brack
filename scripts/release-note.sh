#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <release_branch>"
  exit 1
fi

CURRENT_VERSION=$1
RELEASE_BRANCH="release/$CURRENT_VERSION"

PREV_VERSION=$(gh release list | grep -v "rc" | awk '{print $1}' | sort -V | tail -n 1)

if [ -z "$PREV_VERSION" ]; then
  echo "No previous stable release found." >&2
  exit 1
fi

PREV_DATE=$(gh release view "$PREV_VERSION" --json publishedAt --jq '.publishedAt' 2>/dev/null)

if [ -z "$PREV_DATE" ]; then
  echo "Previous version $PREV_VERSION not found." >&2
  exit 1
fi

echo "Comparing PRs merged between $PREV_VERSION and the new release branch $RELEASE_BRANCH..." >&2

PR_LIST=$(gh pr list --state merged --base develop --search "merged:>$PREV_DATE" --json number,title,mergedAt,author --jq 'sort_by(.number)[] | "* \(.title) by @\(.author.login) in https://github.com/brack-lang/brack/pull/\(.number)"')

if [ -z "$PR_LIST" ]; then
  echo "No PRs were merged between $PREV_VERSION and the release branch $RELEASE_BRANCH." >&2
else
  echo "PRs merged between $PREV_VERSION and the release branch $RELEASE_BRANCH:" >&2
  echo "## What's Changed\n$PR_LIST"
fi

