/*
 * @Date: 2023-04-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-04
 * @FilePath: /algorithm/golang/1000_merge_stones/merge_stones.go
 */

// Package main ...
package main

func mergeStones(stones []int, k int) int {
	n := len(stones)
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	if (n-1)%(k-1) != 0 {
		return -1
	}

	d := make([][]int, n)
	for i := range d {
		d[i] = make([]int, n)
		for j := range d[i] {
			d[i][j] = 1e9
		}
	}
	sum := make([]int, n+1)
	for i, s := 0, 0; i < n; i++ {
		d[i][i] = 0
		s += stones[i]
		sum[i+1] = s
	}

	for len := 2; len <= n; len++ {
		for l := 0; l < n && l+len-1 < n; l++ {
			r := l + len - 1
			for p := l; p < r; p += k - 1 {
				d[l][r] = min(d[l][r], d[l][p]+d[p+1][r])
			}
			if (r-l)%(k-1) == 0 {
				d[l][r] += sum[r+1] - sum[l]
			}
		}
	}
	return d[0][n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		stones := []int{3, 2, 4, 1}
		k := 2
		ans := 20
		assert(mergeStones(stones, k) == ans)
	}

	{
		stones := []int{3, 2, 4, 1}
		k := 3
		ans := -1
		assert(mergeStones(stones, k) == ans)
	}

	{
		stones := []int{3, 5, 1, 2, 6}
		k := 3
		ans := 25
		assert(mergeStones(stones, k) == ans)
	}
}
