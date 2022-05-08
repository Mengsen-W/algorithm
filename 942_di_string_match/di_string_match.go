/*
 * @Date: 2022-05-09 07:38:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-09 07:42:53
 * @FilePath: /algorithm/942_di_string_match/di_string_match.go
 */

package main

import "reflect"

func diStringMatch(s string) []int {
	n := len(s)
	perm := make([]int, n+1)
	lo, hi := 0, n
	for i, ch := range s {
		if ch == 'I' {
			perm[i] = lo
			lo++
		} else {
			perm[i] = hi
			hi--
		}
	}
	perm[n] = lo // 最后剩下一个数，此时 lo == hi
	return perm
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(diStringMatch("IDID"), []int{0, 4, 1, 3, 2})
	assert(diStringMatch("III"), []int{0, 1, 2, 3})
	assert(diStringMatch("DDI"), []int{3, 2, 0, 1})
}
