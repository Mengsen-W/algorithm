// Package main ...
package main

import "sort"

func asteroidsDestroyed(mass int, asteroids []int) bool {
	sort.Ints(asteroids)       // 按照质量升序排序
	currentMass := int64(mass) // 防止整数溢出
	for _, asteroid := range asteroids {
		// 按顺序遍历小行星，尝试摧毁并更新质量或者返回结果
		if currentMass < int64(asteroid) {
			return false
		}
		currentMass += int64(asteroid)
	}
	return true // 成功摧毁所有小行星
}

func main() {
	tests := []struct {
		mass      int
		asteroids []int
		ans       bool
	}{
		{10, []int{3, 9, 19, 5, 21}, true},
		{5, []int{4, 9, 23, 4}, false},
	}

	for _, test := range tests {
		if asteroidsDestroyed(test.mass, test.asteroids) != test.ans {
			println("测试失败")
		}
	}
}
