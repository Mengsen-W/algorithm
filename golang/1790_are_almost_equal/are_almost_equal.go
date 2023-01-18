/*
 * @Date: 2022-10-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-11
 * @FilePath: /algorithm/1790_are_almost_equal/are_almost_equal.go
 */

package main

func areAlmostEqual(s1, s2 string) bool {
	i, j := -1, -1
	for idx := range s1 {
		if s1[idx] != s2[idx] {
			if i < 0 {
				i = idx
			} else if j < 0 {
				j = idx
			} else {
				return false
			}
		}
	}
	return i < 0 || j >= 0 && s1[i] == s2[j] && s1[j] == s2[i]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(areAlmostEqual("bank", "kanb"))
	assert(!areAlmostEqual("attack", "defend"))
	assert(areAlmostEqual("kelb", "kelb"))
	assert(!areAlmostEqual("abcd", "dcba"))
}
