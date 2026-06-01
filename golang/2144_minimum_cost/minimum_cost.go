// Package main ...
package main

import "sort"

func minimumCost(cost []int) int {
	sort.Sort(sort.Reverse(sort.IntSlice(cost)))
	res := 0
	n := len(cost)
	for i := 0; i < n; i++ {
		if i%3 != 2 {
			res += cost[i]
		}
	}
	return res
}

func main() {
	tests := []struct {
		cost []int
		ans  int
	}{
		{[]int{1, 2, 3}, 5},
		{[]int{6, 5, 7, 9, 2, 2}, 23},
		{[]int{5, 5}, 10},
	}

	for _, test := range tests {
		if minimumCost(test.cost) != test.ans {
			println("error")
		}
	}
}
