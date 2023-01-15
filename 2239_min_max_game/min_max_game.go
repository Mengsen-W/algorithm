/*
 * @Date: 2023-01-15
 * @LastEditors: error: git config user.email & please set dead value or install git
 * @LastEditTime: 2023-01-15
 * @FilePath: /algorithm/2239_min_max_game/min_max_game.go
 */

package main

func minMaxGame(nums []int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	n := len(nums)
	if n == 1 {
		return nums[0]
	}
	newNums := make([]int, n/2)
	for i := 0; i < n/2; i++ {
		if i%2 == 0 {
			newNums[i] = min(nums[i*2], nums[i*2+1])
		} else {
			newNums[i] = max(nums[i*2], nums[i*2+1])
		}
	}
	return minMaxGame(newNums)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
    vector<int> nums{1, 3, 5, 2, 4, 8, 2, 2};
    int ans = 1;
    assert(Solution().minMaxGame(nums) == ans);
  }

  {
    vector<int> nums{3};
    int ans = 3;
    assert(Solution().minMaxGame(nums) == ans);
  }
}
