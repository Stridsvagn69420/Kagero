install:
	@go install kagero.go

run:
	@go run kagero.go

build:
	go build -o build/ kagero.go