/*
 * @Date: 2022-09-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-03
 * @FilePath: /algorithm/646_find_longest_chain/find_longest_chain.go
 */

package main

import (
	"math"
	"sort"
)

func findLongestChain(pairs [][]int) (ans int) {
	sort.Slice(pairs, func(i, j int) bool { return pairs[i][1] < pairs[j][1] })
	cur := math.MinInt32
	for _, p := range pairs {
		if cur < p[0] {
			cur = p[1]
			ans++
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
	pairs := [][]int{{1, 2}, {2, 3}, {3, 4}}
	ans := 2
	assert(findLongestChain(pairs) == ans)
}
