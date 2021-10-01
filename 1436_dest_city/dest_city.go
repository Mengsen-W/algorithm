/*
 * @Date: 2021-10-01 09:39:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-01 09:57:02
 */

package main

func destCity(paths [][]string) string {
	citiesA := map[string]bool{}
	for _, path := range paths {
		citiesA[path[0]] = true
	}
	for _, path := range paths {
		if !citiesA[path[1]] {
			return path[1]
		}
	}
	return ""
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		paths := [][]string{{"London", "New York"}, {"New York", "Lima"}, {"Lima", "Sao Paulo"}}
		ans := "Sao Paulo"
		assert(destCity(paths) == ans)
	}
	{
		paths := [][]string{{"B", "C"}, {"D", "B"}, {"C", "A"}}
		ans := "A"
		assert(destCity(paths) == ans)
	}
}
