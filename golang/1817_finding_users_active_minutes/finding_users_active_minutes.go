/*
 * @Date: 2023-01-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-20
 * @FilePath: /algorithm/golang/1817_finding_users_active_minutes/finding_users_active_minutes.go
 */

package main

import "reflect"

func findingUsersActiveMinutes(logs [][]int, k int) []int {
	mp := map[int]map[int]bool{}
	for _, p := range logs {
		id, t := p[0], p[1]
		if mp[id] == nil {
			mp[id] = map[int]bool{}
		}
		mp[id][t] = true
	}
	ans := make([]int, k+1)
	for _, m := range mp {
		ans[len(m)]++
	}
	return ans[1:]
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		logs := [][]int{{0, 5}, {1, 2}, {0, 2}, {0, 5}, {1, 3}}
		k := 5
		ans := []int{0, 2, 0, 0, 0}
		assert(findingUsersActiveMinutes(logs, k), ans)
	}

	{
		logs := [][]int{{1, 1}, {2, 2}, {2, 3}}
		k := 4
		ans := []int{1, 1, 0, 0}
		assert(findingUsersActiveMinutes(logs, k), ans)
	}

}
