# rustgo
demo of call go lib in rust

# build

static:
``` shell
go build -buildmode=c-archive -o libhelloworld.a main.go
```

shared:
``` shell
go build -o libhellowrold.so -buildmode=c-shared main.go
```