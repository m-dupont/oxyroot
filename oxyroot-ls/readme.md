# oxyroot-ls

## Install

    ```bash
    cargo install oxyroot-ls
    ```

## Usage

    ```bash
    oxyroot-ls --file test_suite/create_root_files_with_root/t04_01_write_tree_points.root
    ```

## Example output

```
=== "test_suite/create_root_files_with_root/t04_01_write_tree_points.root" ===
> TTree name='myTree' (title='')
> Data in myTree:
name                           | typename                       | interpretation                
-------------------------------+-------------------------------+-------------------------------
points                         | Point                          | Poi32                         
x                              | int32_t                        | i32                           
y                              | int32_t                        | i32                           

```
