/*
 * @Date: 2022-04-12 09:11:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-12 09:24:46
 * @FilePath: /algorithm/806_number_of_lines/number_of_lines.go
 */

package main

import "reflect"

func numberOfLines(widths []int, s string) []int {
	const maxWidth = 100
	lines, width := 1, 0
	for _, c := range s {
		need := widths[c-'a']
		width += need
		if width > maxWidth {
			lines++
			width = need
		}
	}
	return []int{lines, width}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(numberOfLines(
		[]int{10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10},
		"abcdefghijklmnopqrstuvwxyz",
	),
		[]int{3, 60},
	)

	assert(numberOfLines(
		[]int{4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10},
		"bbbcccdddaaa",
	),
		[]int{2, 4},
	)
}
