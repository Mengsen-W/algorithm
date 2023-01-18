/*
 * @Date: 2021-08-07 13:12:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-07 13:26:38
 */

package main

func circularArrayLoop(nums []int) bool {
	return circularArrayLoop_double_pointer(nums)
	// return circularArrayLoop_bfs(nums)
}

func circularArrayLoop_double_pointer(nums []int) bool {
	n := len(nums)
	next := func(cur int) int {
		return ((cur+nums[cur])%n + n) % n
	}

	for i, num := range nums {
		if num == 0 {
			continue
		}
		slow, fast := i, next(i)
		for nums[slow]*nums[fast] > 0 && nums[slow]*nums[next(fast)] > 0 {
			if slow == fast {
				if slow == next(slow) {
					break
				}
				return true
			}
			slow = next(slow)
			fast = next(next(fast))
		}
		add := i
		for nums[add]*nums[next(add)] > 0 {
			tmp := add
			add = next(add)
			nums[tmp] = 0
		}
	}
	return false
}

func circularArrayLoop_bfs(nums []int) bool {
	const OFFSET int = 100010
	n := len(nums)
	for i := 0; i < n; i++ {
		if nums[i] >= OFFSET {
			continue
		}
		cur := i
		tag := OFFSET + i
		last := -1
		flag := nums[cur] > 0

		for {
			next := ((cur+nums[cur])%n + n) % n
			last = nums[cur]
			nums[cur] = tag
			cur = next
			if cur == i {
				break
			}
			if nums[cur] >= OFFSET {
				break
			}
			if flag && nums[cur] < 0 {
				break
			}
			if !flag && nums[cur] > 0 {
				break
			}
		}
		if last%n != 0 && nums[cur] == tag {
			return true
		}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{2, -1, 1, 2, 2}
		assert(circularArrayLoop_double_pointer(nums))
		assert(circularArrayLoop_bfs(nums))
	}
	{
		nums := []int{-1, 2}
		assert(!circularArrayLoop_double_pointer(nums))
		assert(!circularArrayLoop_bfs(nums))
	}
	{
		nums := []int{2, -1, 1, -2, -2}
		assert(!circularArrayLoop_double_pointer(nums))
		assert(!circularArrayLoop_bfs(nums))
	}
}
