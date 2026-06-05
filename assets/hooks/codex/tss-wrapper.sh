#!/usr/bin/env sh
export TSS_AGENT="${TSS_AGENT:-codex}"
exec tss run -- "$@"
