#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class RangeFreqQuery {
 private:
  // 数值为键，出现下标数组为值的哈希表
  unordered_map<int, vector<int>> occurence;

 public:
  RangeFreqQuery(vector<int>& arr) {
    // 顺序遍历数组初始化哈希表
    int n = arr.size();
    for (int i = 0; i < n; ++i) {
      occurence[arr[i]].push_back(i);
    }
  }

  int query(int left, int right, int value) {
    // 查找对应的出现下标数组，不存在则为空
    const vector<int>& pos = occurence[value];
    // 两次二分查找计算子数组内出现次数
    auto l = lower_bound(pos.begin(), pos.end(), left);
    auto r = upper_bound(pos.begin(), pos.end(), right);
    return r - l;
  }
};

int main() {
  vector<int> input{12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56};
  RangeFreqQuery* rangeFreqQuery = new RangeFreqQuery(input);
  assert(rangeFreqQuery->query(1, 2, 4) == 1);    // 返回 1 。4 在子数组 [33, 4] 中出现 1 次。
  assert(rangeFreqQuery->query(0, 11, 33) == 2);  // 返回 2 。33 在整个子数组中出现 2 次。
}