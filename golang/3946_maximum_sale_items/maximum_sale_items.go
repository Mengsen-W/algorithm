// Package main ...
package main

import "fmt"

func maximumSaleItems(items [][]int, budget int) int {
	f := make([]int, budget+1)
	mn := items[0][1]
	for _, item := range items {
		factor, price := item[0], item[1]
		mn = min(mn, price)
		cnt := 0
		for _, other := range items {
			if other[0]%factor == 0 {
				cnt++
			}
		}
		for j := budget; j >= price; j-- {
			f[j] = max(f[j], f[j-price]+cnt)
		}
	}
	ans := 0
	for j := 0; j <= budget; j++ {
		ans = max(ans, f[j]+(budget-j)/mn)
	}
	return ans
}

func main() {
	tests := []struct {
		items  [][]int
		budget int
		ans    int
	}{
		{[][]int{{6, 2}, {2, 6}, {3, 4}}, 9, 4},
		{[][]int{{2, 4}, {3, 2}, {4, 1}, {6, 4}, {12, 4}}, 8, 10},
	}

	for _, test := range tests {
		if ans := maximumSaleItems(test.items, test.budget); ans != test.ans {
			panic(fmt.Sprintf("test %v failed: got %v, want %v", test, ans, test.ans))
		}
	}
}
