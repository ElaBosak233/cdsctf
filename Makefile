BINARY := cloudsdale
PACKAGE := github.com/elabosak233/cloudsdale

GOOS := $(shell go env GOOS)
GOARCH := $(shell go env GOARCH)

export TERM := xterm-256color
export CGO_ENABLED := 1

GIT_TAG := $(shell git describe --tags --always)
GIT_COMMIT := $(shell git rev-parse HEAD)
GIT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD)

LDFLAGS := -X $(PACKAGE)/internal/global.GitTag=$(GIT_TAG) -X $(PACKAGE)/internal/global.GitCommitID=$(GIT_COMMIT) -X $(PACKAGE)/internal/global.GitBranch=$(GIT_BRANCH)

.PHONY: all build run clean swag

all: build

clean:
	@rm -rf ./build

swag:
	@echo Generating swagger docs...
	swag init -g ./cmd/cloudsdale/main.go -o ./api
	@echo Swagger docs generated.

build: swag
	@echo Building $(PACKAGE)...
	@go build -ldflags "-linkmode external -w -s $(LDFLAGS)" -o ./build/$(BINARY)
	@echo Build finished.

run: swag
	@echo Running $(PACKAGE)...
	go run -ldflags "$(LDFLAGS)" $(PACKAGE)/cmd/cloudsdale
	@echo Run finished.