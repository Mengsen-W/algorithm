// Package main ...
package main

import "fmt"

func maxDistance(colors []int) int {
	n := len(colors)
	res := 0 // 两栋颜色不同房子的最远距离
	// 遍历两栋房子下标并维护最远距离
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			if colors[i] != colors[j] {
				if j-i > res {
					res = j - i
				}
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		colors []int
		ans    int
	}{
		{[]int{1, 1, 1, 6, 1, 1, 1}, 3},
		{[]int{1, 8, 3, 8, 3}, 4},
		{[]int{0, 1}, 1},
	}

	for index, test := range tests {
		if maxDistance(test.colors) != test.ans {
			fmt.Printf("%d\n", index)
		}
	}
}
