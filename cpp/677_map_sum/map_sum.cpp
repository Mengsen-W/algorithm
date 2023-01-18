/*
 * @Date: 2021-11-14 02:00:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-14 02:09:02
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class MapSum {
 public:
  MapSum() {}

  void insert(string key, int val) { cnt[key] = val; }

  int sum(string prefix) {
    int res = 0;
    for (auto& [key, val] : cnt) {
      if (key.substr(0, prefix.size()) == prefix) {
        res += val;
      }
    }
    return res;
  }

 private:
  unordered_map<string, int> cnt;
};

int main() {
  MapSum ms = MapSum();
  ms.insert("apple", 3);
  assert(ms.sum("ap") == 3);
  ms.insert("app", 2);
  assert(ms.sum("ap") == 5);
}