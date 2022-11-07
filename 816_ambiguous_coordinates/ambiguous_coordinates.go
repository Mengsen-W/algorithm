/*
 * @Date: 2022-11-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-07
 * @FilePath: /algorithm/816_ambiguous_coordinates/ambiguous_coordinates.go
 */

package main

import "reflect"

func ambiguousCoordinates(s string) (res []string) {
	getPos := func(s string) (pos []string) {
		if s[0] != '0' || s == "0" {
			pos = append(pos, s)
		}
		for p := 1; p < len(s); p++ {
			if p != 1 && s[0] == '0' || s[len(s)-1] == '0' {
				continue
			}
			pos = append(pos, s[:p]+"."+s[p:])
		}
		return
	}
	n := len(s) - 2
	s = s[1 : len(s)-1]
	for l := 1; l < n; l++ {
		lt := getPos(s[:l])
		if len(lt) == 0 {
			continue
		}
		rt := getPos(s[l:])
		if len(rt) == 0 {
			continue
		}
		for _, i := range lt {
			for _, j := range rt {
				res = append(res, "("+i+", "+j+")")
			}
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(ambiguousCoordinates("(123)"), []string{"(1, 23)", "(1, 2.3)", "(12, 3)", "(1.2, 3)"})
	assert(ambiguousCoordinates("(00011)"), []string{"(0, 0.011)", "(0.001, 1)"})
	assert(ambiguousCoordinates("(0123)"), []string{"(0, 123)", "(0, 1.23)", "(0, 12.3)", "(0.1, 23)", "(0.1, 2.3)", "(0.12, 3)"})
	assert(ambiguousCoordinates("(100)"), []string{"(10, 0)"})
}
