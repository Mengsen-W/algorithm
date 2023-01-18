/*
 * @Date: 2021-06-20 09:49:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-20 10:24:45
 * @FilePath: \algorithm\1600_throne_inheritance\throne_inheritance.go
 * @Description: file content
 */

package main

import "fmt"

type ThroneInheritance struct {
	king  string
	edges map[string][]string
	dead  map[string]bool
}

func Constructor(kingName string) (t ThroneInheritance) {
	return ThroneInheritance{kingName, map[string][]string{}, map[string]bool{}}
}

func (t *ThroneInheritance) Birth(parentName, childName string) {
	t.edges[parentName] = append(t.edges[parentName], childName)
}

func (t *ThroneInheritance) Death(name string) {
	t.dead[name] = true
}

func (t *ThroneInheritance) GetInheritanceOrder() (ans []string) {
	var preorder func(string)
	preorder = func(name string) {
		if !t.dead[name] {
			ans = append(ans, name)
		}
		for _, childName := range t.edges[name] {
			preorder(childName)
		}
	}
	preorder(t.king)
	return
}

func main() {
	t := Constructor("king")
	t.Birth("king", "andy")
	t.Birth("king", "bob")
	t.Birth("king", "catherine")
	t.Birth("andy", "matthew")
	t.Birth("bob", "alex")
	t.Birth("bob", "asha")
	fmt.Printf("%s\n", t.GetInheritanceOrder())
	t.Death("bob")
	fmt.Printf("%s\n", t.GetInheritanceOrder())
}
