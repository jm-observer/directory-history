use crate::compare::compare_files;
use crate::ty::compare::ChangeRecord;
use crate::ty::{Dir, File};
use async_recursion::async_recursion;
use log::warn;
use std::slice::Iter;
use std::vec::IntoIter;

#[async_recursion]
pub async fn compare_dir_v2(
    mut dir_before: &Dir,
    mut dir: &Dir,
) -> (Vec<ChangeRecord>, Vec<(Dir, Dir)>) {
    let sub_dirs = dir.dirs.iter();
    let sub_dirs_before = dir_before.dirs.iter();
    let mut dir_change_records = compare_files(&dir_before.files, &dir.files);

    let (mut dir_change_records_tmp, wait_check_dirs) =
        compare_sub_dirs(sub_dirs_before, sub_dirs).await;
    dir_change_records.append(&mut dir_change_records_tmp);
    (dir_change_records, wait_check_dirs)
}
#[async_recursion]
async fn compare_sub_dirs(
    mut dirs_before: Iter<Dir>,
    mut dirs: Iter<Dir>,
) -> (Vec<ChangeRecord>, Vec<(Dir, Dir)>) {
    let mut dir_change_records = Vec::new();
    let mut dir_op = dirs.next();
    let mut dir_op_before = dirs_before.next();
    let mut sub_dir_compare = Vec::new();
    loop {
        if dir_op.is_none() {
            // 剩下的旧文件都为删除文件
            if let Some(dir_tmp) = dir_op_before {
                dir_change_records.push(ChangeRecord::init_delete_dir_record(dir_tmp.name.clone()));
                while let Some(dir_tmp) = dirs_before.next() {
                    dir_change_records
                        .push(ChangeRecord::init_delete_dir_record(dir_tmp.name.clone()));
                }
            }
            break;
        };
        if dir_op_before.is_none() {
            // 剩下的旧文件都为删除文件
            if let Some(dir_tmp) = dir_op {
                dir_change_records.push(ChangeRecord::init_add_dir_record(dir_tmp.name.clone()));
                while let Some(dir_tmp) = dirs.next() {
                    dir_change_records
                        .push(ChangeRecord::init_add_dir_record(dir_tmp.name.clone()));
                }
            }
            break;
        };
        if let Some(dir_tmp) = dir_op.take() {
            if let Some(dir_before_tmp) = dir_op_before.take() {
                if dir_tmp.name == dir_before_tmp.name {
                    // 需要进一步比较文件夹内的文件
                    sub_dir_compare.push((dir_before_tmp, dir_tmp));
                    dir_op = dirs.next();
                    dir_op_before = dirs_before.next();
                } else if dir_tmp.name < dir_before_tmp.name {
                    // 当前文件为新增文件
                    dir_change_records
                        .push(ChangeRecord::init_add_dir_record(dir_tmp.name.clone()));
                    dir_op = dirs.next();
                    dir_op_before = Some(dir_before_tmp);
                } else {
                    // 该历史文件为删除文件
                    dir_change_records.push(ChangeRecord::init_delete_dir_record(
                        dir_before_tmp.name.clone(),
                    ));
                    dir_op_before = dirs_before.next();
                    dir_op = Some(dir_tmp)
                }
            }
        }
    }
    (dir_change_records, sub_dir_compare)
}
