/*
 * @Date: 2022-03-23 00:37:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-23 01:13:22
 * @FilePath: /algorithm/440_find_kth_number/find_kth_number.go
 */

package main

func findKthNumber(n, k int) int {
	getSteps := func(cur, n int) (steps int) {
		min := func(a, b int) int {
			if a > b {
				return b
			}
			return a
		}
		first, last := cur, cur
		for first <= n {
			steps += min(last, n) - first + 1
			first *= 10
			last = last*10 + 9
		}
		return
	}
	cur := 1
	k--
	for k > 0 {
		steps := getSteps(cur, n)
		if steps <= k {
			k -= steps
			cur++
		} else {
			cur *= 10
			k--
		}
	}
	return cur
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(findKthNumber(13, 2) == 10)
	assert(findKthNumber(1, 1) == 1)
}
