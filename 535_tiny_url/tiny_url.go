/*
 * @Date: 2022-06-29
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-29
 * @FilePath: /algorithm/535_tiny_url/tiny_url.go
 */

package main

import (
	"math/rand"
	"strconv"
	"strings"
)

type Codec map[int]string

func Constructor() Codec {
	return Codec{}
}

func (c Codec) encode(longUrl string) string {
	for {
		key := rand.Int()
		if c[key] == "" {
			c[key] = longUrl
			return "http://tinyurl.com/" + strconv.Itoa(key)
		}
	}
}

func (c Codec) decode(shortUrl string) string {
	i := strings.LastIndexByte(shortUrl, '/')
	key, _ := strconv.Atoi(shortUrl[i+1:])
	return c[key]
}

func main() {
	url := "https://leetcode.com/problems/design-tinyurl"
	obj := Constructor()
	tiny := obj.encode(url)
	ans := obj.decode(tiny)
	if ans != url {
		panic("Not Passed")
	}
}
