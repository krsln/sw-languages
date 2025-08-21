# Go

You can run millions of `concurrent goroutines` in a single program without creating serious performance problems, which
makes Go the perfect choice for high-scale concurrent applications such as `webservers` and `microservices`.

## Go installation

https://go.dev/doc/install

IDE https://www.jetbrains.com/go/?var=1  
open ide, new project -> GOROOT + download (or update SDK)

```shell
go version

mkdir go-example
cd go-example
go mod init go-example

#go mod init          # equivalent to `npm init`
# go.mod file         # equivalent to `package.json`
# go.sum file         # equivalent to `package-lock.json`
go get <package_name> # equivalent to `npm install <package_name>`

go run .
go run main.go

``` 