// Package main ...
package main

import (
	"reflect"
)

type OrderedStream struct {
	stream []string
	ptr    int
}

func Constructor(n int) OrderedStream {
	return OrderedStream{make([]string, n+1), 1}
}

func (s *OrderedStream) Insert(idKey int, value string) []string {
	s.stream[idKey] = value
	start := s.ptr
	for s.ptr < len(s.stream) && s.stream[s.ptr] != "" {
		s.ptr++
	}
	return s.stream[start:s.ptr]
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	o := Constructor(5)
	assert(o.Insert(3, "ccccc"), []string{})
	assert(o.Insert(1, "aaaaa"), []string{"aaaaa"})
	assert(o.Insert(2, "bbbbb"), []string{"bbbbb", "ccccc"})
	assert(o.Insert(5, "eeeee"), []string{})
	assert(o.Insert(4, "ddddd"), []string{"ddddd", "eeeee"})
}
