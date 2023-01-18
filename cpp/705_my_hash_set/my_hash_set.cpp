/*
 * @Date: 2021-03-13 08:32:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-13 08:35:22
 * @FilePath: \algorithm\705_my_hash_set\my_hash_set.cpp
 */

#include <cassert>
#include <cstring>

#define N 1000000 + 1
class MyHashSet {
 public:
  /** Initialize your data structure here. */
  MyHashSet() { memset(set_, false, N); }

  void add(int key) { set_[key] = true; }

  void remove(int key) { set_[key] = false; }

  /** Returns true if this set contains the specified element */
  bool contains(int key) { return set_[key]; }

 private:
  bool set_[N];
};

int main() {
  MyHashSet* obj = new MyHashSet();
  obj->add(1);
  assert(obj->contains(1));
  obj->remove(1);
  assert(!obj->contains(1));
  assert(!obj->contains(2));
  return 0;
}