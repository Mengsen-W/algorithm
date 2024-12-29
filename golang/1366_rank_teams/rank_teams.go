// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func rankTeams(votes []string) string {
	// 初始化哈希映射
	ranking := make(map[byte][]int)
	for i := 0; i < len(votes[0]); i++ {
		ranking[votes[0][i]] = make([]int, len(votes[0]))
	}
	// 遍历统计
	for _, vote := range votes {
		for i := 0; i < len(vote); i++ {
			ranking[vote[i]][i]++
		}
	}

	// 取出所有的键值对
	result := make([]struct {
		vid  byte
		rank []int
	}, 0, len(ranking))
	for k, v := range ranking {
		result = append(result, struct {
			vid  byte
			rank []int
		}{k, v})
	}
	// 排序
	sort.Slice(result, func(i, j int) bool {
		for k := 0; k < len(result[i].rank); k++ {
			if result[i].rank[k] != result[j].rank[k] {
				return result[i].rank[k] > result[j].rank[k]
			}
		}
		return result[i].vid < result[j].vid
	})
	ans := make([]byte, 0, len(result))
	for _, r := range result {
		ans = append(ans, r.vid)
	}
	return string(ans)
}

func main() {
	tests := []struct {
		votes []string
		ans   string
	}{
		{[]string{"ABC", "ACB", "ABC", "ACB", "ACB"}, "ACB"},
		{[]string{"WXYZ", "XYZW"}, "XWYZ"},
		{[]string{"ZMNAGUEDSJYLBOPHRQICWFXTVK"}, "ZMNAGUEDSJYLBOPHRQICWFXTVK"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rankTeams(test.votes), index)
	}
}
