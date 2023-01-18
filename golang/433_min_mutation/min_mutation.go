/*
 * @Date: 2022-05-07 06:31:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-07 06:39:28
 * @FilePath: /algorithm/433_min_mutation/min_mutation.go
 */

package main

func minMutation(start, end string, bank []string) int {
	if start == end {
		return 0
	}

	diffOne := func(s, t string) (diff bool) {
		for i := range s {
			if s[i] != t[i] {
				if diff {
					return false
				}
				diff = true
			}
		}
		return
	}
	m := len(bank)
	adj := make([][]int, m)
	endIndex := -1
	for i, s := range bank {
		if s == end {
			endIndex = i
		}
		for j := i + 1; j < m; j++ {
			if diffOne(s, bank[j]) {
				adj[i] = append(adj[i], j)
				adj[j] = append(adj[j], i)
			}
		}
	}
	if endIndex == -1 {
		return -1
	}

	var q []int
	vis := make([]bool, m)
	for i, s := range bank {
		if diffOne(start, s) {
			q = append(q, i)
			vis[i] = true
		}
	}
	for step := 1; q != nil; step++ {
		tmp := q
		q = nil
		for _, cur := range tmp {
			if cur == endIndex {
				return step
			}
			for _, nxt := range adj[cur] {
				if !vis[nxt] {
					vis[nxt] = true
					q = append(q, nxt)
				}
			}
		}
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(minMutation("AACCGGTT", "AAACGGTA", []string{"AACCGGTA", "AACCGCTA", "AAACGGTA"}) == 2)
	assert(minMutation("AACCGGTT", "AACCGGTA", []string{"AACCGGTA"}) == 1)
	assert(minMutation("AAAAACCC", "AACCCCCC", []string{"AAAACCCC", "AAACCCCC", "AACCCCCC"}) == 3)
}
