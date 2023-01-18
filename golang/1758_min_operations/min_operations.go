/*
 * @Date: 2022-11-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-29
 * @FilePath: /algorithm/1758_min_operations/min_operations.go
 */

package main

func minOperations(s string) int {
	cnt := 0
	for i, c := range s {
		if i%2 != int(c-'0') {
			cnt++
		}
	}
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	return min(cnt, len(s)-cnt)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(minOperations("0100") == 1)
	assert(minOperations("10") == 0)
	assert(minOperations("1111") == 2)
}
