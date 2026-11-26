// Package main ...
package main

import "strconv"

func smallestUniqueSubarray(nums []int) int {
	n := len(nums)

	sa := make([]int, n)
	rank := make([]int, n)
	nextRank := make([]int, n)

	maxRank := 0
	for i, num := range nums {
		sa[i] = i
		rank[i] = num
		maxRank = max(maxRank, num)
	}

	// 倍增构造排序后的后缀数组
	temp := make([]int, n)
	cnt := make([]int, max(n+1, maxRank+2))

	countingSort := func(getKey func(int) int) {
		clear(cnt[:maxRank+2])

		for _, i := range sa {
			cnt[getKey(i)]++
		}

		for i := 1; i <= maxRank+1; i++ {
			cnt[i] += cnt[i-1]
		}

		for p := n - 1; p >= 0; p-- {
			i := sa[p]
			key := getKey(i)
			cnt[key]--
			temp[cnt[key]] = i
		}

		sa, temp = temp, sa
	}

	for k := 1; k < n; k <<= 1 {
		// 先按第二关键字排序，再按第一关键字稳定排序
		countingSort(func(i int) int {
			if i+k < n {
				return rank[i+k] + 1
			}
			return 0
		})
		countingSort(func(i int) int {
			return rank[i] + 1
		})

		nextRank[sa[0]] = 0

		// 给排序后的新后缀分配排名
		for i := 1; i < n; i++ {
			x, y := sa[i-1], sa[i]
			rx, ry := -1, -1
			if x+k < n {
				rx = rank[x+k]
			}
			if y+k < n {
				ry = rank[y+k]
			}

			nextRank[y] = nextRank[x]
			if rank[x] != rank[y] || rx != ry {
				nextRank[y]++
			}
		}

		rank, nextRank = nextRank, rank
		maxRank = rank[sa[n-1]]

		// n 个后缀有 n 个排名，排名各不相同，说明排序完毕
		if maxRank == n-1 {
			break
		}
	}

	// Kasai 算法构造 LCP
	pos := make([]int, n)
	lcp := make([]int, n)
	for i, index := range sa {
		pos[index] = i
	}

	h := 0
	for i := 0; i < n; i++ {
		if pos[i] == 0 {
			h = 0
			continue
		}

		j := sa[pos[i]-1]
		for i+h < n && j+h < n && nums[i+h] == nums[j+h] {
			h++
		}

		lcp[pos[i]] = h

		if h > 0 {
			h--
		}
	}

	answer := n
	for i := 0; i < n; i++ {
		next := 0
		if i+1 < n {
			next = lcp[i+1]
		}

		need := max(lcp[i], next) + 1
		if sa[i]+need <= n {
			answer = min(answer, need)
		}
	}

	return answer
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 3, 3}, 3},
		{[]int{2, 1, 2, 3, 3}, 1},
		{[]int{1, 1, 2, 2, 1}, 2},
	}

	for index, test := range tests {
		if got := smallestUniqueSubarray(test.nums); got != test.ans {
			panic("got " + strconv.Itoa(got) + " expected " + strconv.Itoa(test.ans) + " for test case " + strconv.Itoa(index))
		}
	}
}
