use crate::ty::compare::ChangeRecord;
use crate::ty::{Dir, File};
use async_recursion::async_recursion;
use log::warn;
use std::vec::IntoIter;

#[async_recursion]
pub async fn compare_dir(dir_before: Dir, dir: Dir) -> Vec<ChangeRecord> {
    // debug!(
    //     "比对：{:?}:{}——{:?}:{}",
    //     dir_before.path, dir_before.sha256, dir.path, dir.sha256
    // );
    let sub_dirs = dir.dirs.into_iter();
    let sub_dirs_before = dir_before.dirs.into_iter();
    let mut dir_change_records = Vec::new();

    let mut sub_dirs_records = compare_sub_dirs(sub_dirs_before, sub_dirs).await;
    dir_change_records.append(&mut sub_dirs_records);
    dir_change_records.append(&mut compare_files(&dir_before.files, &dir.files));
    dir_change_records
}
#[async_recursion]
async fn compare_sub_dirs(
    mut dirs_before: IntoIter<Dir>,
    mut dirs: IntoIter<Dir>,
) -> Vec<ChangeRecord> {
    let mut dir_change_records = Vec::new();
    let mut dir_op = dirs.next();
    let mut dir_op_before = dirs_before.next();
    let mut sub_dir_compare = Vec::new();
    loop {
        if dir_op.is_none() {
            // 剩下的旧文件都为删除文件
            if let Some(dir_tmp) = dir_op_before {
                dir_change_records.push(ChangeRecord::init_delete_dir_record(dir_tmp.path));
                while let Some(dir_tmp) = dirs_before.next() {
                    dir_change_records.push(ChangeRecord::init_delete_dir_record(dir_tmp.path));
                }
            }
            break;
        };
        if dir_op_before.is_none() {
            // 剩下的旧文件都为删除文件
            if let Some(dir_tmp) = dir_op {
                dir_change_records.push(ChangeRecord::init_add_dir_record(dir_tmp.path));
                while let Some(dir_tmp) = dirs.next() {
                    dir_change_records.push(ChangeRecord::init_add_dir_record(dir_tmp.path));
                }
            }
            break;
        };
        if let Some(dir_tmp) = dir_op.take() {
            if let Some(dir_before_tmp) = dir_op_before.take() {
                // debug!(
                //     "比对子文件夹：{:?}:{} {:?}:{}",
                //     dir_tmp.name,
                //     dir_tmp.sha256, dir_before_tmp.name, dir_before_tmp.sha256
                // );
                if dir_tmp.name == dir_before_tmp.name {
                    if dir_tmp.sha256 != dir_before_tmp.sha256 {
                        // 需要进一步比较文件夹内的文件
                        sub_dir_compare.push(tokio::spawn(compare_dir(dir_before_tmp, dir_tmp)));
                    }
                    dir_op = dirs.next();
                    dir_op_before = dirs_before.next();
                } else if dir_tmp.name < dir_before_tmp.name {
                    // 当前文件为新增文件
                    dir_change_records
                        .push(ChangeRecord::init_add_dir_record(dir_tmp.path.clone()));
                    dir_op = dirs.next();
                    dir_op_before = Some(dir_before_tmp);
                } else {
                    // 该历史文件为删除文件
                    dir_change_records.push(ChangeRecord::init_delete_dir_record(
                        dir_before_tmp.path.clone(),
                    ));
                    dir_op_before = dirs_before.next();
                    dir_op = Some(dir_tmp)
                }
            }
        }
    }

    for tmp in sub_dir_compare.into_iter() {
        match tmp.await {
            Ok(mut records) => {
                if records.len() > 0 {
                    // debug!("新增差异记录: {}", records.len());
                    dir_change_records.append(&mut records);
                }
            }
            Err(e) => {
                warn!("文件夹比对报错: {:?}", e);
            }
        }
    }
    dir_change_records
}

pub fn compare_files(files_before: &Vec<File>, files: &Vec<File>) -> Vec<ChangeRecord> {
    let mut index_before = 0;
    let mut index = 0;
    let file_num = files.len();
    let file_before_num = files_before.len();
    let mut file: &File;
    let mut file_before: &File;
    let mut file_change_records = Vec::new();
    loop {
        file = if index >= file_num {
            // 剩下的旧文件都为删除文件
            while index_before < file_before_num {
                file_change_records.push(ChangeRecord::init_delete_file_record(
                    files_before[index_before].path.clone(),
                ));
                index_before += 1;
            }
            break;
        } else {
            &files[index]
        };
        file_before = if index_before >= file_before_num {
            // 剩下的新文件都为新增文件
            while index < file_num {
                file_change_records.push(ChangeRecord::init_add_file_record(
                    files[index].path.clone(),
                ));
                index_before += 1;
            }
            break;
        } else {
            &files_before[index_before]
        };
        if file.name == file_before.name {
            if file.sha256 == file.sha256 {
                // 该文件未发生变化
                index += 1;
                index_before += 1;
            } else {
                file_change_records.push(ChangeRecord::init_modiry_file_record(file.path.clone()));
            }
        } else if file.name < file_before.name {
            // 当前文件为新增文件
            index += 1;
            file_change_records.push(ChangeRecord::init_add_file_record(file.path.clone()));
        } else {
            // 该历史文件为删除文件
            index_before += 1;
            file_change_records.push(ChangeRecord::init_delete_file_record(
                file_before.path.clone(),
            ));
        }
    }
    file_change_records
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string_compare() {
        let mut a: Vec<String> = vec!["bcd".to_string(), "abc".to_string()];
        a.sort_by_key(|x| x.clone());
        assert_eq!(a, vec!["abc".to_string(), "bcd".to_string()]);
        let mut a: Vec<String> = vec!["bcd".to_string(), "abc".to_string()];
        a.sort_by(|x, y| x.cmp(y));
        assert_eq!(a, vec!["abc".to_string(), "bcd".to_string()]);
        assert!("abc" < "bcd");
        assert!("abc" < "abd");
    }
}
