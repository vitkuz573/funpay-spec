#!/usr/bin/env bash
set -euo pipefail

WEBSPEC_BIN="${WEBSPEC_BIN:-webspec}"
SPEC_FILE="${1:-spec/funpay.webspec.yaml}"

echo "Validating $SPEC_FILE with $WEBSPEC_BIN..."
"$WEBSPEC_BIN" validate --spec "$SPEC_FILE"
echo "OK: $SPEC_FILE is valid webspec."
