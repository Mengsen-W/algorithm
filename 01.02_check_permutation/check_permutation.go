/*
 * @Date: 2022-09-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-27
 * @FilePath: /algorithm/01.02_check_permutation/check_permutation.go
 */

package main

func CheckPermutation(s1 string, s2 string) bool {
	var c1, c2 [26]int
	for _, ch := range s1 {
		c1[ch-'a']++
	}
	for _, ch := range s2 {
		c2[ch-'a']++
	}
	return c1 == c2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(CheckPermutation("abc", "bca"))
	assert(!CheckPermutation("abc", "bad"))
}
