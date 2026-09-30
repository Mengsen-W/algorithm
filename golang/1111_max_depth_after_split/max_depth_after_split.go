// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func maxDepthAfterSplit(seq string) []int {
	n := len(seq)
	ans := make([]int, n)
	for i := 0; i < n; i++ {
		if seq[i] == '(' {
			ans[i] = (i & 1) ^ 1
		} else {
			ans[i] = (i & 1) ^ 0
		}
	}
	return ans
}

func main() {
	tests := []struct {
		seq string
		ans []int
	}{
		{"(()())", []int{0, 1, 1, 1, 1, 0}},
		{"()(())()", []int{0, 0, 0, 1, 1, 0, 1, 1}},
	}

	for _, test := range tests {
		if res := maxDepthAfterSplit(test.seq); !reflect.DeepEqual(res, test.ans) {
			fmt.Printf("FAIL: for %s, expected %v, got %v\n", test.seq, test.ans, res)
		}
	}
}
