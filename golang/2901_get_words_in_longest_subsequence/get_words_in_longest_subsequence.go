// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getWordsInLongestSubsequence(words []string, groups []int) []string {
	n := len(groups)
	dp := make([]int, n)
	prev := make([]int, n)
	for i := range dp {
		dp[i] = 1
		prev[i] = -1
	}
	maxIndex := 0

	for i := 1; i < n; i++ {
		for j := 0; j < i; j++ {
			if check(words[i], words[j]) && dp[j]+1 > dp[i] && groups[i] != groups[j] {
				dp[i] = dp[j] + 1
				prev[i] = j
			}
		}
		if dp[i] > dp[maxIndex] {
			maxIndex = i
		}
	}

	ans := []string{}
	for i := maxIndex; i >= 0; i = prev[i] {
		ans = append(ans, words[i])
	}
	reverse(ans)
	return ans
}

func check(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	diff := 0
	for i := 0; i < len(s1); i++ {
		if s1[i] != s2[i] {
			diff++
			if diff > 1 {
				return false
			}
		}
	}
	return diff == 1
}

func reverse(arr []string) {
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
}

func main() {
	tests := []struct {
		words  []string
		groups []int
		ans    []string
	}{
		{[]string{"bab", "dab", "cab"}, []int{1, 2, 2}, []string{"bab", "dab"}},
		{[]string{"a", "b", "c", "d"}, []int{1, 2, 3, 4}, []string{"a", "b", "c", "d"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getWordsInLongestSubsequence(test.words, test.groups), index)
	}
}
