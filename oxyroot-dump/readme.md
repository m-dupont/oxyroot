# oxyroot-dump

## Install

    ```bash
    cargo install oxyroot-dump
    ```

## Usage

    ```bash
    oxyroot-dump --file test_suite/create_root_files_with_root/t04_01_write_tree_points.root
    ```

## Example output

```
=== "test_suite/create_root_files_with_root/t04_01_write_tree_points.root" ===
>>> tree: "myTree"
name                           | typename                       | interpretation                
-------------------------------+-------------------------------+-------------------------------
points                         | Point                          | Poi32                         
x                              | int32_t                        | i32                           
y                              | int32_t                        | i32                           
Can not interpret type_name = "Poi32"
[0][x]: 0
[0][y]: 0
[1][x]: 1
[1][y]: 1
[2][x]: 2
[2][y]: 4
[3][x]: 3
[3][y]: 9
[4][x]: 4
[4][y]: 16
[5][x]: 5
[5][y]: 25
[6][x]: 6
[6][y]: 36
[7][x]: 7
[7][y]: 49
[8][x]: 8
[8][y]: 64
[9][x]: 9
[9][y]: 81

```
