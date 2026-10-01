# ACT.mk — Convenience wrappers for running GitHub Actions locally with `act`
# https://github.com/nektos/act
#
# Usage:
#   make -f ACT.mk act-help          Show all targets
#   make -f ACT.mk install-act       Install act (macOS/Linux)
#   make -f ACT.mk act-test          Run the test workflow
#   make -f ACT.mk act-check         Run the main CI workflow (build + test matrix)
#   make -f ACT.mk act-format        Run the format-check workflow
#
# Override defaults:
#   make -f ACT.mk act-check ACT_EXTRA_ARGS="--dryrun"

ACT            := act
UNAME_S        := $(shell uname -s)
UNAME_M        := $(shell uname -m)
GITHUB_TOKEN   := $(shell gh auth token 2>/dev/null)

# Auto-detect Apple Silicon and set container architecture to avoid issues.
ACT_ARCH       :=
ifeq ($(UNAME_S),Darwin)
ifeq ($(UNAME_M),arm64)
	ACT_ARCH := --container-architecture linux/amd64
endif
endif

# Base flags used for every invocation.
ACT_BASE_ARGS  := --rm $(ACT_ARCH)

# GitHub token for workflows that fetch from GitHub inside the container.
ifneq ($(GITHUB_TOKEN),)
	ACT_BASE_ARGS += -s GITHUB_TOKEN="$(GITHUB_TOKEN)"
endif

# Allow user to append extra flags (e.g. --dryrun, --verbose, -P ubuntu-24.04=...).
ACT_EXTRA_ARGS ?=

# Convenience: run a specific job only (e.g. ACT_JOB=build).
ACT_JOB        ?=
ifneq ($(ACT_JOB),)
	ACT_BASE_ARGS += -j $(ACT_JOB)
endif

.PHONY: act-help install-act act-list \
        act-test act-check act-format \
        act-commit-build act-docker-check act-pages \
        act-build-job act-all

act-help:
	@echo "ACT.mk — Run GitHub Actions locally"
	@echo ""
	@echo "Install"
	@echo "  make act-help                    Show this message (also: make -f ACT.mk act-help)"
	@echo "  make install-act                 Install act (Homebrew or curl)"
	@echo ""
	@echo "List / inspect"
	@echo "  make act-list                    List workflows and jobs"
	@echo ""
	@echo "Run individual workflows"
	@echo "  make act-test                    Run .github/workflows/test.yml"
	@echo "  make act-check                   Run .github/workflows/workflow.yml"
	@echo "  make act-format                  Run .github/workflows/format-check.yml"
	@echo "  make act-commit-build            Run .github/workflows/commit-by-commit-build.yml"
	@echo "  make act-docker-check            Run .github/workflows/docker-image-check.yml"
	@echo "  make act-pages                   Run .github/workflows/pages.yml"
	@echo ""
	@echo "Run specific jobs"
	@echo "  make act-build-job               Run only the 'build' job from workflow.yml"
	@echo "  make act-test ACT_JOB=build"
	@echo ""
	@echo "Run everything"
	@echo "  make act-all                     Run all workflows sequentially"
	@echo ""
	@echo "Overrides"
	@echo "  ACT_EXTRA_ARGS=...               Append arbitrary act flags"
	@echo "  ACT_JOB=...                      Run a specific job name"
	@echo "  GITHUB_TOKEN=...                 Provide a PAT (default: \`gh auth token\`)"

install-act:
ifeq ($(shell which act 2>/dev/null),)
ifeq ($(UNAME_S),Darwin)
	@echo "Installing act via Homebrew..."
	brew install act
else
	@echo "Installing act via install script..."
	curl -s https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
endif
else
	@echo "act is already installed: $$(which act)"
	@act --version
endif

act-list:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -l

act-test:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/test.yml

act-check:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/workflow.yml

act-format:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/format-check.yml

act-commit-build:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/commit-by-commit-build.yml

act-docker-check:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/docker-image-check.yml

act-pages:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/pages.yml

# Run only the build job from the main workflow (skips the heavy test matrix).
act-build-job:
	$(ACT) $(ACT_BASE_ARGS) $(ACT_EXTRA_ARGS) -W .github/workflows/workflow.yml -j build

act-all: act-format act-test act-check act-commit-build act-docker-check act-pages
	@echo "✅ All workflows completed"
