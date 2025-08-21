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

go build -o main.exe

```

## Standard Layout

https://github.com/golang-standards/project-layout

```text
myproject/
├── cmd/
│   └── myapp/
│       └── main.go
├── internal/
│   └── app/
│       └── service.go
├── pkg/
│   └── utils/
│       └── helpers.go
├── api/
│   └── api.proto
├── configs/
│   └── config.yaml
├── scripts/
│   └── migrate.sh
├── deployments/
│   └── docker/
│       └── Dockerfile
├── test/
│   └── integration/
├── go.mod
└── README.md

myproject/
├── cmd/                    # Main applications (entry points)
│   └── myapp/             # One folder per executable
│       └── main.go        # Main package with func main()
├── internal/              # Private code (restricted to this project)
│   ├── api/               # API handlers, routes, or controllers
│   ├── config/            # Configuration loading and structs
│   └── models/            # Data models or business logic
├── pkg/                   # Reusable libraries (shareable across projects)
│   └── mylib/             # Example library code
├── go.mod                 # Module definition
├── go.sum                 # Dependency checksums
├── README.md              # Project documentation
├── LICENSE                # License file (e.g., MIT, Apache)
├── Makefile               # Build automation (optional)
├── tests/                 # Additional test files or test data
└── scripts/               # Build, deploy, or utility scripts
```

## References

https://github.com/BekBrace/Go_Course_2024  