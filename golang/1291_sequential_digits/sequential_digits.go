// Package main ...
package main

import (
	"fmt"
	"reflect"
	"sort"
)

func sequentialDigits(low int, high int) []int {
	ans := []int{}
	for i := 1; i <= 9; i++ {
		num := i
		for j := i + 1; j <= 9; j++ {
			num = num*10 + j
			if num >= low && num <= high {
				ans = append(ans, num)
			}
		}
	}
	sort.Ints(ans)

	return ans
}

func main() {
	tests := []struct {
		low  int
		high int
		ans  []int
	}{
		{100, 300, []int{123, 234}},
		{1000, 13000, []int{1234, 2345, 3456, 4567, 5678, 6789, 12345}},
	}

	for _, test := range tests {
		got := sequentialDigits(test.low, test.high)
		if !reflect.DeepEqual(got, test.ans) {
			fmt.Printf("sequentialDigits(%d, %d) = %v, want %v", test.low, test.high, got, test.ans)
		}
	}
}
