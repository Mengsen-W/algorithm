/*
 * @Date: 2022-01-07 01:08:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-07 01:23:09
 */

package main

func maxDepth(s string) (ans int) {
	size := 0
	for _, ch := range s {
		if ch == '(' {
			size++
			if size > ans {
				ans = size
			}
		} else if ch == ')' {
			size--
		}
	}
	return
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(maxDepth("(1+(2*3)+((8)/4))+1"), 3)
	assert(maxDepth("(1)+((2))+(((3)))"), 3)
	assert(maxDepth("1+(2*3)/(2-1)"), 1)
	assert(maxDepth("1"), 0)
}
