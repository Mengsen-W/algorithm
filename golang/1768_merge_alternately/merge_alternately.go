/*
 * @Date: 2022-10-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-23
 * @FilePath: /algorithm/1768_merge_alternately/merge_alternately.go
 */

package main

func mergeAlternately(word1, word2 string) string {
	n, m := len(word1), len(word2)
	ans := make([]byte, 0, n+m)
	for i := 0; i < n || i < m; i++ {
		if i < n {
			ans = append(ans, word1[i])
		}
		if i < m {
			ans = append(ans, word2[i])
		}
	}
	return string(ans)
}

func main() {

	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		word1 := "abc"
		word2 := "pqr"
		ans := "apbqcr"
		assert(mergeAlternately(word1, word2) == ans)
	}

	{
		word1 := "ab"
		word2 := "pqrs"
		ans := "apbqrs"
		assert(mergeAlternately(word1, word2) == ans)
	}

	{
		word1 := "abcd"
		word2 := "pq"
		ans := "apbqcd"
		assert(mergeAlternately(word1, word2) == ans)
	}
}
