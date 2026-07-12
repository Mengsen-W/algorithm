// Package main ...
package main

import (
	"reflect"
	"sort"
)

func arrayRankTransform(arr []int) []int {
	a := append([]int{}, arr...)
	sort.Ints(a)
	ranks := map[int]int{}
	for _, v := range a {
		if _, ok := ranks[v]; !ok {
			ranks[v] = len(ranks) + 1
		}
	}
	for i, v := range arr {
		arr[i] = ranks[v]
	}
	return arr
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	tests := []struct {
		arr []int
		ans []int
	}{
		{[]int{40, 10, 20, 30}, []int{4, 1, 2, 3}},
		{[]int{100, 100, 100}, []int{1, 1, 1}},
		{[]int{37, 12, 28, 9, 100, 56, 80, 5, 12}, []int{5, 3, 4, 2, 8, 6, 7, 1, 3}},
	}

	for _, test := range tests {
		assert(arrayRankTransform(test.arr), test.ans)
	}
}
