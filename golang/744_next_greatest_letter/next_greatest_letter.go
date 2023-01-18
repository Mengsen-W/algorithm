/*
 * @Date: 2022-04-02 23:34:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-02 23:38:25
 * @FilePath: /algorithm/744_next_greatest_letter/next_greatest_letter.go
 */

package main

import "sort"

func nextGreatestLetter(letters []byte, target byte) byte {
	if target >= letters[len(letters)-1] {
		return letters[0]
	}
	i := sort.Search(len(letters)-1, func(i int) bool { return letters[i] > target })
	return letters[i]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'a') == 'c')
	assert(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'c') == 'f')
	assert(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'd') == 'f')
}
