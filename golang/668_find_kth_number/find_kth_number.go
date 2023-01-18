/*
 * @Date: 2022-05-18 22:07:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-18 22:17:18
 * @FilePath: /algorithm/668_find_kth_number/find_kth_number.go
 */

package main

import "sort"

func findKthNumber(m, n, k int) int {
	return sort.Search(m*n, func(x int) bool {
		count := x / n * n
		for i := x/n + 1; i <= m; i++ {
			count += x / i
		}
		return count >= k
	})
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(findKthNumber(3, 3, 5) == 3)
	assert(findKthNumber(2, 3, 6) == 6)
}
