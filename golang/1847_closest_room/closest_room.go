// Package main ...
package main

import (
	"math"
	"slices"
	"testing"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
	"github.com/stretchr/testify/assert"
)

func closestRoom(rooms [][]int, queries [][]int) []int {
	// 按照 size 从大到小排序
	slices.SortFunc(rooms, func(a, b []int) int { return b[1] - a[1] })

	q := len(queries)
	queryIds := make([]int, q)
	for i := range queryIds {
		queryIds[i] = i
	}
	// 按照 minSize 从大到小排序
	slices.SortFunc(queryIds, func(i, j int) int { return queries[j][1] - queries[i][1] })

	ans := make([]int, q)
	for i := range ans {
		ans[i] = -1
	}
	roomIDs := redblacktree.New[int, struct{}]() // import "github.com/emirpasic/gods/v2/trees/redblacktree"
	j := 0
	for _, i := range queryIds {
		preferredID, minSize := queries[i][0], queries[i][1]
		for j < len(rooms) && rooms[j][1] >= minSize {
			roomIDs.Put(rooms[j][0], struct{}{})
			j++
		}

		diff := math.MaxInt
		// 左边的差
		if node, ok := roomIDs.Floor(preferredID); ok {
			diff = preferredID - node.Key
			ans[i] = node.Key
		}
		// 右边的差
		if node, ok := roomIDs.Ceiling(preferredID); ok && node.Key-preferredID < diff {
			ans[i] = node.Key
		}
	}
	return ans
}

func main() {
	tests := []struct {
		rooms   [][]int
		queries [][]int
		ans     []int
	}{
		{[][]int{{2, 2}, {1, 2}, {3, 2}}, [][]int{{3, 1}, {3, 3}, {5, 2}}, []int{3, -1, 3}},
		{[][]int{{1, 4}, {2, 3}, {3, 5}, {4, 1}, {5, 2}}, [][]int{{2, 3}, {2, 4}, {2, 5}}, []int{2, 1, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, closestRoom(test.rooms, test.queries), index)
	}
}
