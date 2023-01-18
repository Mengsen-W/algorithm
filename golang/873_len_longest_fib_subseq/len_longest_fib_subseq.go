/*
 * @Date: 2022-07-09
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-09
 * @FilePath: /algorithm/873_len_longest_fib_subseq/len_longest_fib_subseq.go
 */

package main

func lenLongestFibSubseq(arr []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	n := len(arr)
	indices := make(map[int]int, n)
	for i, x := range arr {
		indices[x] = i
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	for i, x := range arr {
		for j := n - 1; j >= 0 && arr[j]*2 > x; j-- {
			if k, ok := indices[x-arr[j]]; ok {
				dp[j][i] = max(dp[k][j]+1, 3)
				ans = max(ans, dp[j][i])
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(lenLongestFibSubseq([]int{1, 2, 3, 4, 5, 6, 7, 8}) == 5)
	assert(lenLongestFibSubseq([]int{1, 3, 7, 11, 12, 14, 18}) == 3)
}
