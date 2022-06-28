/*
 * @Date: 2022-06-28
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-28
 * @FilePath: /algorithm/324_wiggle_sort/wiggle_sort.go
 */

package main

import (
	"math/rand"
	"reflect"
	"time"
)

func wiggleSort(nums []int) {
	n := len(nums)
	x := (n + 1) / 2
	rand.Seed(time.Now().UnixNano())
	target := quickSelect(nums, 0, n-1, x-1)
	for k, i, j := 0, 0, n-1; k <= j; k++ {
		if nums[k] > target {
			for j > k && nums[j] > target {
				j--
			}
			nums[k], nums[j] = nums[j], nums[k]
			j--
		}
		if nums[k] < target {
			nums[k], nums[i] = nums[i], nums[k]
			i++
		}
	}
	arr := append([]int{}, nums...)
	for i, j, k := 0, x-1, n-1; i < n; i += 2 {
		nums[i] = arr[j]
		if i+1 < n {
			nums[i+1] = arr[k]
		}
		j--
		k--
	}
}

func quickSelect(a []int, l, r, index int) int {
	q := randomPartition(a, l, r)
	if q == index {
		return a[q]
	}
	if q < index {
		return quickSelect(a, q+1, r, index)
	}
	return quickSelect(a, l, q-1, index)
}

func randomPartition(a []int, l, r int) int {
	i := rand.Intn(r-l+1) + l
	a[i], a[r] = a[r], a[i]
	return partition(a, l, r)
}

func partition(a []int, l, r int) int {
	pivot := a[r]
	i := l - 1
	for j := l; j < r; j++ {
		if a[j] < pivot {
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
		nums := []int{1, 5, 1, 1, 6, 4}
		ans := []int{1, 6, 1, 5, 1, 4}
		wiggleSort(nums)
		assert(nums, ans)
	}

	{
		nums := []int{1, 3, 2, 2, 3, 1}
		ans := []int{2, 3, 1, 3, 1, 2}
		wiggleSort(nums)
		assert(nums, ans)
	}
}
