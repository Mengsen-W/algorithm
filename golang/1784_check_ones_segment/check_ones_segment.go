/*
 * @Date: 2022-10-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-03
 * @FilePath: /algorithm/1784_check_ones_segment/check_ones_segment.go
 */

package main

import "strings"

func checkOnesSegment(s string) bool {
	return !strings.Contains(s, "01")
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(!checkOnesSegment("1001"))
	assert(checkOnesSegment("110"))
}
