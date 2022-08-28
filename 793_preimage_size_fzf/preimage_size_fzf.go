/*
 * @Date: 2022-08-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-28
 * @FilePath: /algorithm/793_preimage_size_fzf/preimage_size_fzf.go
 */

package main

import "sort"

func preimageSizeFZF(k int) int {
	nx := func(k int) int {
		zeta := func(n int) (res int) {
			for n > 0 {
				n /= 5
				res += n
			}
			return
		}
		return sort.Search(5*k, func(x int) bool { return zeta(x) >= k })
	}
	return nx(k+1) - nx(k)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(preimageSizeFZF(0) == 5)
	assert(preimageSizeFZF(5) == 0)
	assert(preimageSizeFZF(3) == 5)
}
