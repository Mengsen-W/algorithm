// Package main ...
package main

import "fmt"

func uniqueXorTriplets(nums []int) int {
	m := 0
	for _, v := range nums {
		m = max(m, v)
	}
	u := 1
	for u <= m {
		u <<= 1
	}
	one := make([]bool, u)
	two := make([]bool, u)
	three := make([]bool, u)
	for _, v := range nums {
		one[v] = true
		for x := 0; x < u; x++ {
			if one[x] {
				two[x^v] = true
			}
		}
	}
	for _, v := range nums {
		for x := 0; x < u; x++ {
			if two[x] {
				three[x^v] = true
			}
		}
	}
	ans := 0
	for _, b := range three {
		if b {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3}, 2},
		{[]int{6, 7, 8, 9}, 4},
	}

	for _, test := range tests {
		if uniqueXorTriplets(test.nums) != test.ans {
			fmt.Println("test failed")
		}
	}
}
