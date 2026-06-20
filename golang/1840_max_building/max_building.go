// Package main ...
package main

import (
	"fmt"
	"sort"
)

func maxBuilding(n int, restrictions [][]int) int {
	// 增加限制 (1, 0)
	restrictions = append(restrictions, []int{1, 0})
	r := restrictions

	// 按位置排序
	sort.Slice(r, func(i, j int) bool {
		return r[i][0] < r[j][0]
	})
	// 增加限制 (n, n-1)
	if r[len(r)-1][0] != n {
		r = append(r, []int{n, n - 1})
	}
	m := len(r)

	// 从左向右传递限制
	for i := 1; i < m; i++ {
		dist := r[i][0] - r[i-1][0]
		if r[i-1][1]+dist < r[i][1] {
			r[i][1] = r[i-1][1] + dist
		}
	}

	// 从右向左传递限制
	for i := m - 2; i >= 0; i-- {
		dist := r[i+1][0] - r[i][0]
		if r[i+1][1]+dist < r[i][1] {
			r[i][1] = r[i+1][1] + dist
		}
	}

	ans := 0
	for i := 0; i < m-1; i++ {
		// 计算 r[i][0] 和 r[i][1] 之间的建筑的最大高度
		dist := r[i+1][0] - r[i][0]
		best := (dist + r[i][1] + r[i+1][1]) / 2
		if best > ans {
			ans = best
		}
	}

	return ans
}

func main() {
	tests := []struct {
		n            int
		restrictions [][]int
		ans          int
	}{
		{5, [][]int{{2, 1}, {4, 1}}, 2},
		{6, [][]int{}, 5},
		{10, [][]int{{5, 3}, {2, 5}, {7, 4}, {10, 3}}, 5},
	}

	for _, test := range tests {
		ans := maxBuilding(test.n, test.restrictions)
		if ans != test.ans {
			fmt.Println("test failed", test.n, test.restrictions, test.ans, ans)
		}
	}
}
