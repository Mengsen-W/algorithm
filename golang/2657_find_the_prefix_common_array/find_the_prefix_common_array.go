// Package main ...
package main

import (
	"fmt"
	"math/bits"
	"reflect"
)

func findThePrefixCommonArray(A []int, B []int) []int {
	var p, q uint64 = 0, 0
	for i := 0; i < len(A); i++ {
		p |= 1 << uint(A[i])
		q |= 1 << uint(B[i])
		A[i] = bits.OnesCount64(p & q)
	}
	return A
}

func main() {
	tests := []struct {
		A   []int
		B   []int
		ans []int
	}{
		{[]int{1, 3, 2, 4}, []int{3, 1, 2, 4}, []int{0, 2, 3, 4}},
		{[]int{2, 3, 1}, []int{3, 1, 2}, []int{0, 1, 3}},
	}

	for _, test := range tests {
		if got := findThePrefixCommonArray(test.A, test.B); !reflect.DeepEqual(got, test.ans) {
			fmt.Println(test.A, test.B, got, test.ans)
		}
	}
}
