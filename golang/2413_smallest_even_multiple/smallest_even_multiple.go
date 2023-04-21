/*
 * @Date: 2023-04-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-21
 * @FilePath: /algorithm/golang/2413_smallest_even_multiple/smallest_even_multiple.go
 */

// Package main ...
package main

func smallestEvenMultiple(n int) int {
	if n%2 == 0 {
		return n
	}
	return n * 2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(smallestEvenMultiple(5) == 10)
	assert(smallestEvenMultiple(6) == 6)
}
