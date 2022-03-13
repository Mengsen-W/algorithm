/*
 * @Date: 2022-03-13 00:28:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-13 00:38:06
 * @FilePath: /algorithm/393_valid_utf8/valid_utf8.go
 */

package main

const mask1, mask2 = 1 << 7, 1<<7 | 1<<6

func getBytes(num int) int {
	if num&mask1 == 0 {
		return 1
	}
	n := 0
	for mask := mask1; num&mask != 0; mask >>= 1 {
		n++
		if n > 4 {
			return -1
		}
	}
	if n >= 2 {
		return n
	}
	return -1
}

func validUtf8(data []int) bool {
	for index, m := 0, len(data); index < m; {
		n := getBytes(data[index])
		if n < 0 || index+n > m {
			return false
		}
		for _, ch := range data[index+1 : index+n] {
			if ch&mask2 != mask1 {
				return false
			}
		}
		index += n
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(validUtf8([]int{197, 130, 1}))
	assert(!validUtf8([]int{235, 140, 4}))
}
