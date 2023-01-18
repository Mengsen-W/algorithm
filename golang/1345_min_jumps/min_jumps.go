/*
 * @Date: 2022-01-22 08:47:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-22 08:56:19
 */

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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minJumps([]int{100, -23, -23, 404, 100, 23, 23, 23, 3, 404}) ==
		3)
	assert(minJumps([]int{7}) == 0)
	assert(minJumps([]int{7, 6, 9, 6, 9, 6, 9, 7}) == 1)
	assert(minJumps([]int{6, 1, 9}) == 2)
	assert(minJumps([]int{11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13}) == 3)
}
