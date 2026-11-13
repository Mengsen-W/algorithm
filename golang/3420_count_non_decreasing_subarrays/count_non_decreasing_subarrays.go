// Package main ...
package main

import (
	"container/heap"
	"strconv"
)

func countNonDecreasingSubarrays(nums []int, k int) int64 {
	n := len(nums)
	var ans int64 = 0
	var operations int64 = 0
	// 优先队列维护窗口最大值
	h := &MaxHeap{}
	heap.Init(h)
	// 记录被该下标覆盖的元素集合
	cnt := make([][]int, n)
	for i := range cnt {
		cnt[i] = []int{}
	}
	// 右侧第一个大于等于的下标
	pos := make([]int, n)
	for i := range pos {
		pos[i] = n
	}
	// 递减单调栈
	st := []int{}

	left := 0
	for right := 0; right < n; right++ {
		// 维护单调栈
		for len(st) > 0 && nums[right] >= nums[st[len(st)-1]] {
			pos[st[len(st)-1]] = right
			st = st[:len(st)-1]
		}
		// 栈顶即为 nums[right] 的左边最近的更大元素
		if len(st) > 0 {
			cnt[st[len(st)-1]] = append(cnt[st[len(st)-1]], right)
		}
		st = append(st, right)

		// 优先队首元素即为窗口最大值
		heap.Push(h, [2]int{nums[right], right})
		// 窗口加入 nums[right] 操作次数需要增加
		if diff := (*h)[0][0] - nums[right]; diff > 0 {
			operations += int64(diff)
		}

		// 最小操作次数超过 k，此时需要缩小窗口
		for operations > int64(k) {
			// 移除窗口左侧元素 nums[left]，操作次数需要较少
			for _, i := range cnt[left] {
				// 移除贡献
				operations -= int64(nums[left]-nums[i]) * int64(min(pos[i], right+1)-i)
			}
			left++
		}

		// 从优先队列中移除小于下标 left 的元素
		for h.Len() > 0 && (*h)[0][1] < left {
			heap.Pop(h)
		}

		// 以 right 为结尾的合法连续子数组数目为 right - left + 1
		ans += int64(right - left + 1)
	}

	return ans
}

type MaxHeap [][2]int

func (h MaxHeap) Len() int            { return len(h) }
func (h MaxHeap) Less(i, j int) bool  { return h[i][0] > h[j][0] }
func (h MaxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x interface{}) { *h = append(*h, x.([2]int)) }
func (h *MaxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
      {[]int{6, 3, 1, 2, 4, 4}, 7, 17},
      {[]int{6, 3, 1, 3, 6}, 4, 12},
	}

	for index, test := range tests {
		if got := countNonDecreasingSubarrays(test.nums, test.k); got != test.ans {
			panic("index:" + strconv.Itoa(index) + " got:" + strconv.Itoa(int(got)) + " want:" + strconv.Itoa(int(test.ans)))
		}
	}
}
