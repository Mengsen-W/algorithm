/*
 * @Date: 2023-04-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-23
 * @FilePath: /algorithm/golang/1105_min_height_shelves/min_height_shelves.go
 */

// Package main ...
package main

func minHeightShelves(books [][]int, shelfWidth int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	n := len(books)
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = 1000000
	}
	dp[0] = 0
	for i := 0; i < n; i++ {
		maxHeight, curWidth := 0, 0
		for j := i; j >= 0; j-- {
			curWidth += books[j][0]
			if curWidth > shelfWidth {
				break
			}
			maxHeight = max(maxHeight, books[j][1])
			dp[i+1] = min(dp[i+1], dp[j]+maxHeight)
		}
	}
	return dp[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		books := [][]int{{1, 1}, {2, 3}, {2, 3}, {1, 1}, {1, 1}, {1, 1}, {1, 2}}
		shelfWidth := 4
		ans := 6
		assert(minHeightShelves(books, shelfWidth) == ans)
	}

	{
		books := [][]int{{1, 3}, {2, 4}, {3, 2}}
		shelfWidth := 6
		ans := 4
		assert(minHeightShelves(books, shelfWidth) == ans)
	}
}
