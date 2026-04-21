/*
 * @Date: 2022-07-13
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-13
 * @FilePath: /algorithm/735_asteroid_collision/asteroid_collision.rs
 */

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];

    for asteroid in asteroids {
        if asteroid < 0 {
            let mut remain = true;

            while stack.len() > 0 && stack[stack.len() - 1] > 0 {
                let top = stack[stack.len() - 1];

                if asteroid.abs() < top {
                    remain = false;
                    break;
                } else if asteroid.abs() == top {
                    remain = false;
                    stack.pop();
                    break;
                } else {
                    stack.pop();
                }
            }

            if remain {
                stack.push(asteroid);
            }
        } else {
            stack.push(asteroid);
        }
    }

    stack
}

fn main() {
    {
        let asteroids = vec![5, 10, -5];
        let ans = vec![5, 10];
        assert_eq!(asteroid_collision(asteroids), ans);
    }

    {
        let asteroids = vec![8, -8];
        let ans = vec![];
        assert_eq!(asteroid_collision(asteroids), ans);
    }

    {
        let asteroids = vec![10, 2, -5];
        let ans = vec![10];
        assert_eq!(asteroid_collision(asteroids), ans);
    }
}
