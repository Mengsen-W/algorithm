###
 # @Date: 2021-08-25 12:07:19
 # @Author: Mengsen Wang
 # @LastEditors: Mengsen Wang
 # @LastEditTime: 2021-08-25 22:37:39
 # @FilePath: /bulaji/scripts/delete_temp_file.sh
###


# delete single file out binary
find $(dirname "$PWD") -name "*.out" | xargs rm -rf

# delete pyc
find $(dirname "$PWD") -name "*.pyc" | xargs rm -rf