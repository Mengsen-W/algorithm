/*
 * @Date: 2022-02-28 02:47:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-28 03:02:48
 * @FilePath: /algorithm/1601_maximum_requests/maximum_requests.go
 */

package main

import "math/bits"

func maximumRequests1(n int, requests [][]int) (ans int) {
	delta := make([]int, n)
	cnt, zero := 0, n
	var dfs func(int)
	dfs = func(pos int) {
		if pos == len(requests) {
			if zero == n && cnt > ans {
				ans = cnt
			}
			return
		}

		// 不选 requests[pos]
		dfs(pos + 1)

		// 选 requests[pos]
		z := zero
		cnt++
		r := requests[pos]
		x, y := r[0], r[1]
		if delta[x] == 0 {
			zero--
		}
		delta[x]--
		if delta[x] == 0 {
			zero++
		}
		if delta[y] == 0 {
			zero--
		}
		delta[y]++
		if delta[y] == 0 {
			zero++
		}
		dfs(pos + 1)
		delta[y]--
		delta[x]++
		cnt--
		zero = z
	}
	dfs(0)
	return
}

func maximumRequests2(n int, requests [][]int) (ans int) {
next:
	for mask := 0; mask < 1<<len(requests); mask++ {
		cnt := bits.OnesCount(uint(mask))
		if cnt <= ans {
			continue
		}
		delta := make([]int, n)
		for i, r := range requests {
			if mask>>i&1 == 1 {
				delta[r[0]]++
				delta[r[1]]--
			}
		}
		for _, d := range delta {
			if d != 0 {
				continue next
			}
		}
		ans = cnt
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		input := [][]int{{0, 1}, {1, 0}, {0, 1}, {1, 2}, {2, 0}, {3, 4}}
		assert(maximumRequests1(5, input) == 5)
		assert(maximumRequests2(5, input) == 5)
	}

	{
		input := [][]int{{0, 0}, {1, 2}, {2, 1}}
		assert(maximumRequests1(5, input) == 3)
		assert(maximumRequests2(5, input) == 3)
	}

	{
		input := [][]int{{0, 3}, {3, 1}, {1, 2}, {2, 0}}
		assert(maximumRequests1(5, input) == 4)
		assert(maximumRequests2(5, input) == 4)
	}
}
