/*
 * @Date: 2022-12-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-05
 * @FilePath: /algorithm/1687_box_delivering/box_delivering.go
 */

package main

func boxDelivering(boxes [][]int, portsCount int, maxBoxes int, maxWeight int) int {
	n := len(boxes)
	p := make([]int, n+1)
	w := make([]int, n+1)
	neg := make([]int, n+1)
	W := make([]int, n+1)
	for i := 1; i <= n; i++ {
		p[i] = boxes[i-1][0]
		w[i] = boxes[i-1][1]
		if i > 1 {
			neg[i] = neg[i-1]
			if p[i-1] != p[i] {
				neg[i]++
			}
		}
		W[i] = W[i-1] + w[i]
	}

	opt := []int{0}
	f := make([]int, n+1)
	g := make([]int, n+1)

	for i := 1; i <= n; i++ {
		for i-opt[0] > maxBoxes || W[i]-W[opt[0]] > maxWeight {
			opt = opt[1:]
		}

		f[i] = g[opt[0]] + neg[i] + 2

		if i != n {
			g[i] = f[i] - neg[i+1]
			for len(opt) > 0 && g[i] <= g[opt[len(opt)-1]] {
				opt = opt[:len(opt)-1]
			}
			opt = append(opt, i)
		}
	}

	return f[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		boxes := [][]int{{1, 1}, {2, 1}, {1, 1}}
		portsCount := 2
		maxBoxes := 3
		maxWeight := 3
		ans := 4
		assert(boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans)
	}

	{
		boxes := [][]int{{1, 2}, {3, 3}, {3, 1}, {3, 1}, {2, 4}}
		portsCount := 3
		maxBoxes := 3
		maxWeight := 6
		ans := 6
		assert(boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans)
	}

	{
		boxes := [][]int{{1, 4}, {1, 2}, {2, 1}, {2, 1}, {3, 2}, {3, 4}}
		portsCount := 3
		maxBoxes := 6
		maxWeight := 7
		ans := 6
		assert(boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans)
	}

	{
		boxes := [][]int{{2, 4}, {2, 5}, {3, 1}, {3, 2}, {3, 7}, {3, 1}, {4, 4}, {1, 3}, {5, 2}}
		portsCount := 5
		maxBoxes := 5
		maxWeight := 7
		ans := 14
		assert(boxDelivering(boxes, portsCount, maxBoxes, maxWeight) == ans)
	}
}
