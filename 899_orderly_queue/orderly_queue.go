/*
 * @Date: 2022-08-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-03
 * @FilePath: /algorithm/899_orderly_queue/orderly_queue.go
 */

package main

import "sort"

func orderlyQueue(s string, k int) string {
	if k == 1 {
		ans := s
		for i := 1; i < len(s); i++ {
			s = s[1:] + s[:1]
			if s < ans {
				ans = s
			}
		}
		return ans
	}
	t := []byte(s)
	sort.Slice(t, func(i, j int) bool { return t[i] < t[j] })
	return string(t)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(orderlyQueue("cba", 1) == "acb")
	assert(orderlyQueue("baaca", 3) == "aaabc")
}
