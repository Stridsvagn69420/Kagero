install:
	@go install src/kagero.go

test:
	@go run src/kagero.go

windows:
	go build -o build/kagero.exe src/kagero.go

linux:
	go build -o build/kagero src/kagero.go