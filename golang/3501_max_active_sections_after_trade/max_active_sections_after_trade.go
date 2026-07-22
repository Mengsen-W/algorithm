// Package main ...
package main

import (
	"fmt"
	"math"
	"reflect"
	"sort"
)

func maxActiveSectionsAfterTrade(s string, queries [][]int) []int {
	n, m := len(s), len(queries)
	cnt1 := 0
	for _, c := range s {
		if c == '1' {
			cnt1++
		}
	}
	// left[i]：表示以位置 i 结尾，与 s[i] 相同的连续区块长度
	left := make([]int, n)
	// right[i]：表示以位置 i 开始，与 s[i] 相同的连续区块长度
	right := make([]int, n)

	for i := 0; i < n; i++ {
		if i > 0 && s[i-1] == s[i] {
			left[i] = left[i-1] + 1
		} else {
			left[i] = 1
		}
	}
	for i := n - 1; i >= 0; i-- {
		if i < n-1 && s[i+1] == s[i] {
			right[i] = right[i+1] + 1
		} else {
			right[i] = 1
		}
	}

	ans := make([]int, m)
	for i := range ans {
		ans[i] = -1
	}
	block_size := int(math.Sqrt(float64(n)))
	// 长度大于块长的询问
	longQueries := make([][4]int, 0, m)

	bruteForce := func(l, r int) int {
		i := l
		best := 0
		prev := math.MinInt32

		for i <= r {
			start := i
			for i <= r && s[i] == s[start] {
				i++
			}
			if s[start] == '0' {
				cur := i - start
				if prev != math.MinInt32 && prev+cur > best {
					best = prev + cur
				}
				prev = cur
			}
		}
		return best
	}

	for i := 0; i < m; i++ {
		l, r := queries[i][0], queries[i][1]
		if r-l+1 > block_size {
			longQueries = append(longQueries, [4]int{l / block_size, l, r, i})
		} else {
			// 长度小于块长的询问，暴力计算
			ans[i] = cnt1 + bruteForce(l, r)
		}
	}

	// 以询问左端点所在块的 ID 为第一关键字，询问右端点为第二关键字排序
	sort.Slice(longQueries, func(i, j int) bool {
		if longQueries[i][0] != longQueries[j][0] {
			return longQueries[i][0] < longQueries[j][0]
		}
		return longQueries[i][2] < longQueries[j][2]
	})

	// 使用数组模拟双端队列，从中间开始扩展，避免频繁内存分配
	subZeroBlocks := make([]int, n)
	head, tail := n/2, n/2
	L, R, bestGain := 0, 0, 0

	for i := 0; i < len(longQueries); i++ {
		bid, l, r, qid := longQueries[i][0], longQueries[i][1], longQueries[i][2], longQueries[i][3]
		if i == 0 || bid > longQueries[i-1][0] {
			// 遍历到一个新的块, 进行初始化操作
			L = (bid+1)*block_size - 1 // L 初始化为该块右端点
			R = (bid + 1) * block_size // R 初始化为下一块左端点
			head, tail = n/2, n/2
			bestGain = 0
		}

		for R <= r {
			sz := right[R]
			if r-R+1 < sz {
				sz = r - R + 1
			}
			if s[R] == '0' {
				if tail > head && s[R-1] == '0' {
					subZeroBlocks[tail-1] += sz
				} else {
					subZeroBlocks[tail] = sz
					tail++
				}
				if tail-head >= 2 {
					val := subZeroBlocks[tail-1] + subZeroBlocks[tail-2]
					if val > bestGain {
						bestGain = val
					}
				}
			}
			R += sz
		}

		// 移动左端点 L 前，备份 bestGain 的值
		tmp_bestGain := bestGain
		// 移动左端点前，subZeroBlocks第一个元素（如果有）的值
		tmp_firstValue := -1
		if tail > head {
			tmp_firstValue = subZeroBlocks[head]
		}
		// 记录移动左端点 L 的过程中，从左侧加入的数字数量
		cnt := 0

		for L >= l {
			sz := left[L]
			if L-l+1 < sz {
				sz = L - l + 1
			}
			if s[L] == '0' {
				if tail > head && s[L+1] == '0' {
					subZeroBlocks[head] += sz
				} else {
					head--
					subZeroBlocks[head] = sz
					cnt++
				}
				if tail-head >= 2 {
					val := subZeroBlocks[head] + subZeroBlocks[head+1]
					if val > bestGain {
						bestGain = val
					}
				}
			}
			L -= sz
		}

		// 回答询问
		ans[qid] = bestGain + cnt1
		// 还原左端点 L
		L = (bid+1)*block_size - 1
		// 还原 bestGain
		bestGain = tmp_bestGain
		// 还原 subZeroBlocks
		head += cnt
		if tmp_firstValue != -1 {
			subZeroBlocks[head] = tmp_firstValue
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s       string
		queries [][]int
		ans     []int
	}{
		{"01", [][]int{{0, 1}}, []int{1}},
		{"0100", [][]int{{0, 3}, {0, 2}, {1, 3}, {2, 3}}, []int{4, 3, 1, 1}},
		{"1000100", [][]int{{1, 5}, {0, 6}, {0, 4}}, []int{6, 7, 2}},
		{"01010", [][]int{{0, 3}, {1, 4}, {1, 3}}, []int{4, 4, 2}},
	}

	for _, test := range tests {
		ans := maxActiveSectionsAfterTrade(test.s, test.queries)
		if !reflect.DeepEqual(ans, test.ans) {
			fmt.Println("Test failed:", test.s, test.queries, ans, test.ans)
		}
	}
}
