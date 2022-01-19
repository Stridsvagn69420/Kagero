install:
	@go install src/kagero.go

test:
	@go run src/kagero.go

run-win:
	./build/kagero.exe

run-linux:
	./build/kagero

windows:
	go build -o build/kagero.exe src/kagero.go

linux:
	go build -o build/kagero src/kagero.go