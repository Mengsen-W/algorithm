#include <bitset>
#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  string findDifferentBinaryString(vector<string>& nums) {
    int n = nums.size();
    // 预处理对应整数的哈希集合
    unordered_set<int> vals;
    for (const string& num : nums) {
      vals.insert(stoi(num, nullptr, 2));
    }
    // 寻找第一个不在哈希集合中的整数
    int val = 0;
    while (vals.count(val)) {
      ++val;
    }
    // 将整数转化为二进制字符串返回
    return bitset<16>(val).to_string().substr(16 - n, 16);
  }
};

int main() {
  vector<tuple<vector<string>, string>> tests{
      {{"01", "10"}, "00"},
      {{"00", "01"}, "10"},
      {{"111", "011", "000"}, "101"},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().findDifferentBinaryString(nums) == expected);
  }
}