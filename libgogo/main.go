package main

import (
	"C"
	"fmt"
	"time"
)

//export start
func start(name *C.char) uint64 {
	index++
	ch := make(chan string)
	goName := C.GoString(name)
	go func() {
		for {
			fmt.Println("!", goName)
			now := time.Now()
			ch <- fmt.Sprintf("[%s]%s", goName, now.String())
		}
	}()
	services[index] = ch
	return index
}

//export get
func get(index uint64) *C.char {
	return C.CString(<-services[index])
}

var index = uint64(0)
var services map[uint64]chan string = make(map[uint64]chan string)

//export plus
func plus(a uint64, b uint64) uint64 {
	return a + b
}

func main() {}
