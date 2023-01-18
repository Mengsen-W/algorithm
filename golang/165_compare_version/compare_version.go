/*
 * @Date: 2021-09-01 13:49:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-01 14:28:17
 */

package main

func compareVersion(version1, version2 string) int {
	n, m := len(version1), len(version2)
	i, j := 0, 0
	for i < n || j < m {
		x := 0
		for ; i < n && version1[i] != '.'; i++ {
			x = x*10 + int(version1[i]-'0')
		}
		i++ // 跳过点号
		y := 0
		for ; j < m && version2[j] != '.'; j++ {
			y = y*10 + int(version2[j]-'0')
		}
		j++ // 跳过点号
		if x > y {
			return 1
		}
		if x < y {
			return -1
		}
	}
	return 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		version1 := "1.01"
		version2 := "1.001"
		assert(compareVersion(version1, version2) == 0)
	}
	{
		version1 := "1.0"
		version2 := "1.0.0"
		assert(compareVersion(version1, version2) == 0)
	}
	{
		version1 := "0.1"
		version2 := "1.1"
		assert(compareVersion(version1, version2) == -1)
	}
	{
		version1 := "1.0.1"
		version2 := "1"
		assert(compareVersion(version1, version2) == 1)
	}
	{
		version1 := "7.5.2.4"
		version2 := "7.5.3"
		assert(compareVersion(version1, version2) == -1)
	}
}
