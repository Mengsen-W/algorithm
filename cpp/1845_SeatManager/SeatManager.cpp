#include <vector>

using namespace std;

class SeatManager {
 public:
  vector<int> available;

  SeatManager(int n) {
    for (int i = 1; i <= n; ++i) {
      available.push_back(i);
    }
  }

  int reserve() {
    pop_heap(available.begin(), available.end(), greater<int>());
    int tmp = available.back();
    available.pop_back();
    return tmp;
  }

  void unreserve(int seatNumber) {
    available.push_back(seatNumber);
    push_heap(available.begin(), available.end(), greater<int>());
  }
};

int main() {
  SeatManager *seatManager = new SeatManager(5);  // 初始化 SeatManager ，有 5 个座位。
  seatManager->reserve();                         // 所有座位都可以预约，所以返回最小编号的座位，也就是 1 。
  seatManager->reserve();                         // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
  seatManager->unreserve(2);                      // 将座位 2 变为可以预约，现在可预约的座位为 [2,3,4,5] 。
  seatManager->reserve();                         // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
  seatManager->reserve();                         // 可以预约的座位为 [3,4,5] ，返回最小编号的座位，也就是 3 。
  seatManager->reserve();                         // 可以预约的座位为 [4,5] ，返回最小编号的座位，也就是 4 。
  seatManager->reserve();                         // 唯一可以预约的是座位 5 ，所以返回 5 。
  seatManager->unreserve(5);                      // 将座位 5 变为可以预约，现在可预约的座位为 [5] 。
}