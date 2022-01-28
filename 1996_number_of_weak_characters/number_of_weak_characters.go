/*
 * @Date: 2022-01-28 01:23:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-28 01:43:04
 */

package main

import "sort"

func numberOfWeakCharacters(properties [][]int) (ans int) {
	sort.Slice(properties, func(i, j int) bool {
		p, q := properties[i], properties[j]
		return p[0] < q[0] || p[0] == q[0] && p[1] > q[1]
	})
	st := []int{}
	for _, p := range properties {
		for len(st) > 0 && st[len(st)-1] < p[1] {
			st = st[:len(st)-1]
			ans++
		}
		st = append(st, p[1])
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(numberOfWeakCharacters([][]int{{5, 5}, {6, 3}, {3, 6}}) == 0)
	assert(numberOfWeakCharacters([][]int{{2, 2}, {3, 3}}) == 1)
	assert(numberOfWeakCharacters([][]int{{1, 5}, {10, 4}, {4, 3}}) == 1)
}
