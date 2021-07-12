/*
 * @Date: 2021-07-12 08:28:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:43:45
 */

#include <algorithm>
#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class TimeMap {
  unordered_map<string, vector<pair<int, string>>> m;

 public:
  TimeMap() {}

  void set(string key, string value, int timestamp) {
    m[key].emplace_back(timestamp, value);
  }

  string get(string key, int timestamp) {
    auto& pairs = m[key];
    // 使用一个大于所有 value 的字符串，以确保在 pairs 中含有 timestamp
    // 的情况下也返回大于 timestamp 的位置
    pair<int, string> p = {timestamp, string({127})};
    auto i = upper_bound(pairs.begin(), pairs.end(), p);
    if (i != pairs.begin()) {
      return (i - 1)->second;
    }
    return "";
  }
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */

int main() {
  {
    TimeMap* tv = new TimeMap{};
    tv->set("foo", "bar", 1);
    assert(tv->get("foo", 1) == "bar");
    assert(tv->get("foo", 3) == "bar");
    tv->set("foo", "bar2", 4);
    assert(tv->get("foo", 4) == "bar2");
    assert(tv->get("foo", 5) == "bar2");
    delete tv;
  }
  {
    TimeMap* tv = new TimeMap{};
    tv->set("love", "high", 10);
    tv->set("love", "low", 20);
    assert(tv->get("love", 5) == "");
    assert(tv->get("love", 10) == "high");
    assert(tv->get("love", 15) == "high");
    assert(tv->get("love", 20) == "low");
    delete tv;
  }
}