/*
 * @Date: 2023-04-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-22
 * @FilePath: /algorithm/golang/1027_longest_arith_seq_length/longest_arith_seq_length.go
 */

// Package main ...
package main

func longestArithSeqLength(nums []int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	minv, maxv := nums[0], nums[0]
	for _, num := range nums[1:] {
		if num < minv {
			minv = num
		} else if num > maxv {
			maxv = num
		}
	}
	diff := maxv - minv
	ans := 1
	for d := -diff; d <= diff; d++ {
		f := make([]int, maxv+1)
		for i := range f {
			f[i] = -1
		}
		for _, num := range nums {
			prev := num - d
			if prev >= minv && prev <= maxv && f[prev] != -1 {
				f[num] = max(f[num], f[prev]+1)
				ans = max(ans, f[num])
			}
			f[num] = max(f[num], 1)
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("not Passed")
		}
	}

	{
		nums := []int{3, 6, 9, 12}
		ans := 4
		assert(longestArithSeqLength(nums) == ans)
	}

	{
		nums := []int{9, 4, 7, 2, 10}
		ans := 3
		assert(longestArithSeqLength(nums) == ans)
	}

	{
		nums := []int{20, 1, 15, 3, 10, 5, 8}
		ans := 4
		assert(longestArithSeqLength(nums) == ans)
	}
}
