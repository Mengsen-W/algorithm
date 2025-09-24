// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const (
	draw     = 0
	mouseWin = 1
	catWin   = 2
)

func catMouseGame(graph [][]int) int {
	n := len(graph)
	dp := make([][][]int, n)
	for i := range dp {
		dp[i] = make([][]int, n)
		for j := range dp[i] {
			dp[i][j] = make([]int, n*(n-1)*2)
			for k := range dp[i][j] {
				dp[i][j][k] = -1
			}
		}
	}

	var getResult, getNextResult func(int, int, int) int
	getResult = func(mouse, cat, turns int) int {
		if turns == n*(n-1)*2 {
			return draw
		}
		res := dp[mouse][cat][turns]
		if res != -1 {
			return res
		}
		if mouse == 0 {
			res = mouseWin
		} else if cat == mouse {
			res = catWin
		} else {
			res = getNextResult(mouse, cat, turns)
		}
		dp[mouse][cat][turns] = res
		return res
	}
	getNextResult = func(mouse, cat, turns int) int {
		curMove := mouse
		if turns%2 == 1 {
			curMove = cat
		}
		defaultRes := mouseWin
		if curMove == mouse {
			defaultRes = catWin
		}
		res := defaultRes
		for _, next := range graph[curMove] {
			if curMove == cat && next == 0 {
				continue
			}
			nextMouse, nextCat := mouse, cat
			if curMove == mouse {
				nextMouse = next
			} else if curMove == cat {
				nextCat = next
			}
			nextRes := getResult(nextMouse, nextCat, turns+1)
			if nextRes != defaultRes {
				res = nextRes
				if res != draw {
					break
				}
			}
		}
		return res
	}
	return getResult(1, 2, 0)
}

func main() {
	tests := []struct {
		graph [][]int
		ans   int
	}{
		{[][]int{{2, 5}, {3}, {0, 4, 5}, {1, 4, 5}, {2, 3}, {0, 2, 3}}, 0},
		{[][]int{{1, 3}, {0}, {3}, {0, 2}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, catMouseGame(test.graph), index)
	}
}
