/*
 * @Date: 2023-02-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-26
 * @FilePath: /algorithm/golang/1255_max_score_words/max_score_words.go
 */

package main

func maxScoreWords(words []string, letters []byte, score []int) (ans int) {
	left := [26]int{}
	for _, c := range letters {
		left[c-'a']++
	}

	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}

	var dfs func(int, int)
	dfs = func(i, total int) {
		if i < 0 { // base case
			ans = max(ans, total)
			return
		}

		// 不选 words[i]
		dfs(i-1, total)

		// 选 words[i]
		for j, c := range words[i] {
			c -= 'a'
			if left[c] == 0 { // 剩余字母不足
				for _, c := range words[i][:j] { // 撤销
					left[c-'a']++
				}
				return
			}
			left[c]--         // 减少剩余字母
			total += score[c] // 累加得分
		}

		dfs(i-1, total)

		// 恢复现场
		for _, c := range words[i] {
			left[c-'a']++
		}
	}
	dfs(len(words)-1, 0)
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		words := []string{"dog", "cat", "dad", "good"}
		letters := []byte{'a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'}
		score := []int{1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
		ans := 23
		assert(maxScoreWords(words, letters, score) == ans)
	}

	{
		words := []string{"xxxz", "ax", "bx", "cx"}
		letters := []byte{'z', 'a', 'b', 'c', 'x', 'x', 'x'}
		score := []int{4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10}
		ans := 27
		assert(maxScoreWords(words, letters, score) == ans)
	}

	{
		words := []string{"leetcode"}
		letters := []byte{'l', 'e', 't', 'c', 'o', 'd'}
		score := []int{0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0}
		ans := 0
		assert(maxScoreWords(words, letters, score) == ans)
	}
}
