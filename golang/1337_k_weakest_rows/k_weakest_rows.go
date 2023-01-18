/*
 * @Date: 2021-08-01 15:01:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-01 15:17:10
 */

package main

import (
	"math/rand"
	"reflect"
	"sort"
	"time"
)

type pair struct{ pow, idx int }

func kWeakestRows(mat [][]int, k int) []int {
	m := len(mat)
	pairs := make([]pair, m)
	for i, row := range mat {
		pow := sort.Search(len(row), func(j int) bool { return row[j] == 0 })
		pairs[i] = pair{pow, i}
	}
	rand.Seed(time.Now().UnixNano())
	randomizedSelected(pairs, 0, m-1, k)
	pairs = pairs[:k]
	sort.Slice(pairs, func(i, j int) bool {
		a, b := pairs[i], pairs[j]
		return a.pow < b.pow || a.pow == b.pow && a.idx < b.idx
	})
	ans := make([]int, k)
	for i, p := range pairs {
		ans[i] = p.idx
	}
	return ans
}

func randomizedSelected(a []pair, l, r, k int) {
	if l >= r {
		return
	}
	pos := randomPartition(a, l, r)
	num := pos - l + 1
	if k == num {
		return
	}
	if k < num {
		randomizedSelected(a, l, pos-1, k)
	} else {
		randomizedSelected(a, pos+1, r, k-num)
	}
}

func randomPartition(a []pair, l, r int) int {
	i := rand.Intn(r-l+1) + l
	a[i], a[r] = a[r], a[i]
	return partition(a, l, r)
}

func partition(a []pair, l, r int) int {
	pivot := a[r]
	i := l - 1
	for j := l; j < r; j++ {
		if a[j].pow < pivot.pow || a[j].pow == pivot.pow && a[j].idx <= pivot.idx {
			i++
			a[i], a[j] = a[j], a[i]
		}
	}
	a[i+1], a[r] = a[r], a[i+1]
	return i + 1
}

func main() {

	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		mat := [][]int{{1, 1, 0, 0, 0},
			{1, 1, 1, 1, 0},
			{1, 0, 0, 0, 0},
			{1, 1, 0, 0, 0},
			{1, 1, 1, 1, 1}}
		k := 3
		ans := []int{2, 0, 3}
		assert(kWeakestRows(mat, k), ans)
	}
	{
		mat := [][]int{
			{1, 0, 0, 0}, {1, 1, 1, 1}, {1, 0, 0, 0}, {1, 0, 0, 0}}
		k := 2
		ans := []int{0, 2}
		assert(kWeakestRows(mat, k), ans)
	}
}
