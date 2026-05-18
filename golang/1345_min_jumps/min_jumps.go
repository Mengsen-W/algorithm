// Package main ...
package main

func minJumps(arr []int) int {
	n := len(arr)
	idx := map[int][]int{}
	for i, v := range arr {
		idx[v] = append(idx[v], i)
	}
	vis := map[int]bool{0: true}
	type pair struct{ idx, step int }
	q := []pair{{}}
	for {
		p := q[0]
		q = q[1:]
		i, step := p.idx, p.step
		if i == n-1 {
			return step
		}
		for _, j := range idx[arr[i]] {
			if !vis[j] {
				vis[j] = true
				q = append(q, pair{j, step + 1})
			}
		}
		delete(idx, arr[i])
		if !vis[i+1] {
			vis[i+1] = true
			q = append(q, pair{i + 1, step + 1})
		}
		if i > 0 && !vis[i-1] {
			vis[i-1] = true
			q = append(q, pair{i - 1, step + 1})
		}
	}
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{
			[]int{100, -23, -23, 404, 100, 23, 23, 23, 3, 404},
			3,
		},
		{
			[]int{7},
			0,
		},
		{
			[]int{7, 6, 9, 6, 9, 6, 9, 7},
			1,
		},
		{
			[]int{6, 1, 9},
			2,
		},
		{
			[]int{11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13},
			3,
		},
	}

	for _, test := range tests {
		if test.ans != minJumps(test.arr) {
			panic("Not Passed")
		}
	}
}
