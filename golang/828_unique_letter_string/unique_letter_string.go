/*
 * @Date: 2022-09-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-06
 * @FilePath: /algorithm/828_unique_letter_string/unique_letter_string.go
 */

package main

func uniqueLetterString(s string) (ans int) {
	idx := map[rune][]int{}
	for i, c := range s {
		idx[c] = append(idx[c], i)
	}
	for _, arr := range idx {
		arr = append(append([]int{-1}, arr...), len(s))
		for i := 1; i < len(arr)-1; i++ {
			ans += (arr[i] - arr[i-1]) * (arr[i+1] - arr[i])
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(uniqueLetterString("ABC") == 10)
	assert(uniqueLetterString("ABA") == 8)
	assert(uniqueLetterString("LEETCODE") == 92)
}
