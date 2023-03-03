/*
 * @Date: 2023-03-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-03
 * @FilePath: /algorithm/golang/1487_get_folder_names/get_folder_names.go
 */

package main

import (
	"reflect"
	"strconv"
)

func getFolderNames(names []string) []string {
	ans := make([]string, len(names))
	index := map[string]int{}
	for p, name := range names {
		i := index[name]
		if i == 0 {
			index[name] = 1
			ans[p] = name
			continue
		}
		for index[name+"("+strconv.Itoa(i)+")"] > 0 {
			i++
		}
		t := name + "(" + strconv.Itoa(i) + ")"
		ans[p] = t
		index[name] = i + 1
		index[t] = 1
	}
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		names := []string{"pes", "fifa", "gta", "pes(2019)"}
		ans := []string{"pes", "fifa", "gta", "pes(2019)"}
		assert(getFolderNames(names), ans)
	}

	{
		names := []string{"gta", "gta(1)", "gta", "avalon"}
		ans := []string{"gta", "gta(1)", "gta(2)", "avalon"}
		assert(getFolderNames(names), ans)
	}

	{
		names := []string{"onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece"}
		ans := []string{"onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece(4)"}
		assert(getFolderNames(names), ans)
	}

	{
		names := []string{"wano", "wano", "wano", "wano"}
		ans := []string{"wano", "wano(1)", "wano(2)", "wano(3)"}
		assert(getFolderNames(names), ans)
	}

	{
		names := []string{"kaido", "kaido(1)", "kaido", "kaido(1)"}
		ans := []string{"kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"}
		assert(getFolderNames(names), ans)
	}
}
