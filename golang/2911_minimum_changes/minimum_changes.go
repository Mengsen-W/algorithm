// Package main ...
package main

import "fmt"

func segmentCost(s string, l, m int, factors [][]int) int {
	best := m
	for _, d := range factors[m] {
		groups := m / d // 每组有 groups 个字符
		changes := 0
		for offset := 0; offset < d; offset++ {
			for t := 0; t < groups/2; t++ { // 组内首尾配对
				if s[l+offset+t*d] != s[l+offset+(groups-1-t)*d] {
					changes++
				}
			}
		}
		best = min(best, changes)
	}
	return best
}

func minimumChanges(s string, k int) int {
	n := len(s)
	factors := make([][]int, n+1)
	// 预处理每个数的真因子
	for d := 1; d <= n/2; d++ {
		for m := d * 2; m <= n; m += d {
			factors[m] = append(factors[m], d)
		}
	}
	cost := make([][]int, n)
	for l := range cost {
		cost[l] = make([]int, n)
	}
	for l := 0; l+1 < n; l++ {
		for r := l + 1; r < n; r++ { // 子串长度至少为 2
			cost[l][r] = segmentCost(s, l, r-l+1, factors)
		}
	}

	memo := make([][]int, k+1)
	for seg := range memo {
		memo[seg] = make([]int, n)
		for start := range memo[seg] {
			memo[seg][start] = -1
		}
	}
	var solve func(seg, start int) int
	solve = func(seg, start int) int {
		if seg == 1 { // 只剩一段
			return cost[start][n-1]
		}
		if memo[seg][start] != -1 { // 之前算过
			return memo[seg][start]
		}
		best := n
		limit := n - 1 - 2*(seg-1) // 第一段之后还要留出 seg-1 段
		for e := start + 1; e <= limit; e++ {
			best = min(best, cost[start][e]+solve(seg-1, e+1))
		}
		memo[seg][start] = best
		return best
	}
	return solve(k, 0)
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans int
	}{
		{"abcac", 2, 1},
		{"abcdef", 2, 2},
		{"aabbaa", 3, 0},
	}

	for _, test := range tests {
		if got := minimumChanges(test.s, test.k); got != test.ans {
			panic(fmt.Sprintf("test %v failed: got %v, want %v", test, got, test.ans))
		}
	}
}
