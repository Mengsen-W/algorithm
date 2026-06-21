// Package main ...
package main

import "fmt"

func maxIceCream(costs []int, coins int) (ans int) {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	const mx int = 1e5
	freq := [mx + 1]int{}
	for _, c := range costs {
		freq[c]++
	}
	for i := 1; i <= mx && coins >= i; i++ {
		c := min(freq[i], coins/i)
		ans += c
		coins -= i * c
	}
	return
}

func main() {
	tests := []struct {
		costs []int
		coins int
		ans   int
	}{
		{[]int{1, 3, 2, 4, 1}, 7, 4},
		{[]int{10, 6, 8, 7, 7, 8}, 5, 0},
		{[]int{1, 6, 3, 1, 2, 5}, 20, 6},
	}

	for _, test := range tests {
		if maxIceCream(test.costs, test.coins) != test.ans {
			fmt.Println("test failed")
		}
	}
}
