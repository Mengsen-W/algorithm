/*
 * @Date: 2022-07-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-23
 * @FilePath: /algorithm/115_sequence_reconstruction/sequence_reconstruction.go
 */

package main

func sequenceReconstruction(nums []int, sequences [][]int) bool {
	n := len(nums)
	g := make([][]int, n+1)
	inDeg := make([]int, n+1)
	for _, sequence := range sequences {
		for i := 1; i < len(sequence); i++ {
			x, y := sequence[i-1], sequence[i]
			g[x] = append(g[x], y)
			inDeg[y]++
		}
	}

	q := []int{}
	for i := 1; i <= n; i++ {
		if inDeg[i] == 0 {
			q = append(q, i)
		}
	}
	for len(q) > 0 {
		if len(q) > 1 {
			return false
		}
		x := q[0]
		q = q[1:]
		for _, y := range g[x] {
			if inDeg[y]--; inDeg[y] == 0 {
				q = append(q, y)
			}
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 2, 3}
		sequences := [][]int{{1, 2}, {1, 3}}
		ans := false
		assert(sequenceReconstruction(nums, sequences) == ans)
	}

	{
		nums := []int{1, 2, 3}
		sequences := [][]int{{1, 2}}
		ans := false
		assert(sequenceReconstruction(nums, sequences) == ans)
	}

	{
		nums := []int{1, 2, 3}
		sequences := [][]int{{1, 2}, {1, 3}, {2, 3}}
		ans := true
		assert(sequenceReconstruction(nums, sequences) == ans)
	}
}
