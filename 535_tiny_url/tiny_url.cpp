/*
 * @Date: 2022-06-29
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-29
 * @FilePath: /algorithm/535_tiny_url/tiny_url.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 private:
  unordered_map<int, string> dataBase;

 public:
  Solution() { srand(time(0)); }

  string encode(string longUrl) {
    int key;
    while (true) {
      key = rand();
      if (dataBase.count(key) == 0) {
        break;
      }
    }
    dataBase[key] = longUrl;
    return string("http://tinyurl.com/") + to_string(key);
  }

  string decode(string shortUrl) {
    int p = shortUrl.rfind('/') + 1;
    int key = stoi(shortUrl.substr(p, int(shortUrl.size()) - p));
    return dataBase[key];
  }
};

int main() {
  string url = "https://leetcode.com/problems/design-tinyurl";
  Solution obj{};
  string tiny = obj.encode(url);
  string ans = obj.decode(tiny);
  assert(ans == url);
}