/*
 * @Date: 2024-04-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-15
 * @FilePath: /algorithm/cpp/706_MyHashMap/MyHashMap.cpp
 */

#include <cassert>
#include <list>
#include <vector>

using namespace std;

class MyHashMap {
 private:
  vector<list<pair<int, int>>> data;
  static const int base = 769;
  static int hash(int key) { return key % base; }

 public:
  /** Initialize your data structure here. */
  MyHashMap() : data(base) {}

  /** value will always be non-negative. */
  void put(int key, int value) {
    int h = hash(key);
    for (auto it = data[h].begin(); it != data[h].end(); it++) {
      if ((*it).first == key) {
        (*it).second = value;
        return;
      }
    }
    data[h].push_back(make_pair(key, value));
  }

  /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
  int get(int key) {
    int h = hash(key);
    for (auto it = data[h].begin(); it != data[h].end(); it++) {
      if ((*it).first == key) {
        return (*it).second;
      }
    }
    return -1;
  }

  /** Removes the mapping of the specified value key if this map contains a mapping for the key */
  void remove(int key) {
    int h = hash(key);
    for (auto it = data[h].begin(); it != data[h].end(); it++) {
      if ((*it).first == key) {
        data[h].erase(it);
        return;
      }
    }
  }
};

int main() {
  MyHashMap myHashMap = MyHashMap();
  myHashMap.put(1, 1);             // myHashMap 现在为 [[1,1]]
  myHashMap.put(2, 2);             // myHashMap 现在为 [[1,1], [2,2]]
  assert(myHashMap.get(1) == 1);   // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
  assert(myHashMap.get(3) == -1);  // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
  myHashMap.put(2, 1);             // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
  assert(myHashMap.get(2) == 1);   // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
  myHashMap.remove(2);             // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
  assert(myHashMap.get(2) == -1);  // 返回 -1（未找到），myHashMap 现在为 [[1,1]]
}