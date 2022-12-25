/*
 * @Date: 2022-12-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-24
 * @FilePath: /algorithm/1754_largest_merge/largest_merge.go
 */
package main

import "fmt"

func largestMerge(word1, word2 string) string {
	merge := []byte{}
	i, j, n, m := 0, 0, len(word1), len(word2)
	for i < n || j < m {
		// fmt.Printf("i: %v j: %v\n", i, j)
		fmt.Printf("word1[i:]: %v word2[j:]: %v\n", word1[i:], word2[j:])
		if i < n && word1[i:] > word2[j:] {
			merge = append(merge, word1[i])
			i += 1
		} else {
			merge = append(merge, word2[j])
			j += 1
		}
	}
	return string(merge)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	// {
	// 	word1, word2 := "cabaa", "bcaaa"
	// 	ans := "cbcabaaaaa"
	// 	assert(largestMerge(word1, word2) == ans)
	// }

	{
		word1, word2 := "abcabc", "abdcaba"
		ans := "abdcabcabcaba"
		assert(largestMerge(word1, word2) == ans)
	}
}
