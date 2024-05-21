use std::{fs, path::PathBuf};

use clap::Parser;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use tqdm::tqdm;
use tree_sitter::{Point, TreeCursor};
use walkdir::WalkDir;

// tree-sitter가 코드를 분석/parsing해 만든 tree는 굉장히 복잡하다.
// 그 안 각각의 node는 타입, 코드 시작 위치, 종료 위치, 코드 내용 등을 가지고있다.
// line_comment [0, 0] - [0, 14]
// identifier [4, 3] - [4, 10]
// 뭐 이런 식으로 생겼다.
// 그래서 이 정보를 저장하기 위한 구조체를 만들어준다.
// 이 구조체는 타입, 시작 위치, 종료 위치, 타입..?을 저장한다.
// type TypePosInfo = (String, usize, usize, String);

type TypePosInfo<'a> = (&'a str, usize, usize, Point, Point);

// vertical로 자식 node 중 첫 번째 것을 찾으면, 그걸 horizontal로 탐색.
// horizontal로 탐색 = 자식 node와 그 sibling들을 찾는다는 뜻.
// 다 찾으면 parent로 돌아간다.
fn visit_vertical(source_code: &str, cursor: &mut TreeCursor, acc: &mut Vec<TypePosInfo>) {
    if cursor.goto_first_child() {
        visit_horizontal(source_code, cursor, acc);
        cursor.goto_parent();
    }
}

// horizontal로 탐색한다. next sibling 함수를 이용.
// 각 자식들의 또 다른 자식들을 vertical로 찾아낸다.
// 다 찾아내면 horizontal을 부른 vertical로 돌아갈테니, 그러면 parent가 있는 한 단계 위로 돌아가고
// 매우 자연스럽게 돌아간 parent의 다음 sibling을 horizontal로 찾고 있을 것이다.
fn visit_horizontal(source_code: &str, cursor: &mut TreeCursor, acc: &mut Vec<TypePosInfo>) {
    loop {
        //find_type(source_code, cursor, acc);
        find_type_except_comment(source_code, cursor, acc);

        visit_vertical(source_code, cursor, acc);

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}
// 모든 탐색이 끝나면, acc엔 해당 source code에서 찾아 정보를 기록한 모든 type들이 Vec<TypePosInfo>에 저장될 것이다.
// 여기서 타입에 맞춰 변이를 하면 된다.
// 이건 굳이 쓰진 않기로.
/*
pub fn find_type(source_code: &str, cursor: &mut TreeCursor, acc: &mut Vec<TypePosInfo>) {
    let node = cursor.node();
    let type_string = node.kind();
    match type_string {
        // https://github.com/tree-sitter/tree-sitter-rust/blob/b77c0d8ac28a7c143224e6ed9b4f9e4bd044ff5b/src/node-types.json#L393-L464
        "abstract_type"
        | "array_type"
        | "bounded_type"
        | "dynamic_type"
        | "function_type"
        | "generic_type"
        | "macro_invocation"
        | "metavariable"
        | "never_type"
        | "pointer_type"
        | "reference_type"
        | "removed_trait_bound"
        | "scoped_type_identifier"
        | "tuple_type"
        | "identifier"
        | "unit_type"
        | "type_identifier" => {
            // pub fn block_on<Copy> 여기서 <Copy> 이런 것들을 찾아내는 것
            let start_byte = node.start_byte();
            let end_byte = node.end_byte();
            let start_point = node.start_position();
            let end_point = node.end_position();

            let _struct_name = node;
            // dbg!(&struct_name);

            // avoid unicode-byte index mismatch problem
            // - just ignore them
            let source_chars: Vec<char> = source_code.chars().collect();
            if source_chars.len() <= end_byte - 1 {
                return;
            }

            let type_info: TypePosInfo =
                (type_string, start_byte, end_byte, start_point, end_point);
            acc.push(type_info);
        }
        _ => {} // Other node kinds can be handled as needed
    }
}
*/

// 이쪽에서 type을 수정하는 작업을 진행한다
/*
pub fn modify_types_my(source_code: &str, structs: &Vec<TypePosInfo>) -> Vec<String> {
    let mut new_exprs: HashMap<&str, Vec<&str>> = HashMap::new();
    new_exprs.insert("type_identifier", vec!["", "i32", "str", "Copy"]);
    new_exprs.insert("unit_type", vec!["", "f32", "char", "Clone", "a"]);
    new_exprs.insert("identifier", vec!["", "f32", "char", "Clone", "a"]);
    // Add more type mappings as needed

    let mut modified_versions = Vec::new();

    for &(type_string, start_byte, end_byte, start_point, end_point) in structs.iter() {
        // start_byte와 end_byte를 이용해 원본 코드를 자른다.
        // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
        let before = &source_code[..start_byte];
        let after = &source_code[end_byte..];
        let original = &source_code[start_byte..end_byte];

        // type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.
        if let Some(exprs) = new_exprs.get(type_string) {
            for n in exprs {
                modified_versions.push(format!("{}{}{}", before, n, after));
                // print difference between original and modified version
                // _name is not a code. find it with before and after.
                if n.to_string() != "" {
                    println!(
                        "{}-{} {} : {} -> {}",
                        start_point, end_point, type_string, original, n
                    );
                }
            }
        }
    }

    //변형된 버전을 돌려줌.
    modified_versions
}
*/

//comment를 제외한 모든 타입에 대해 돌려준다.
pub fn find_type_except_comment(
    source_code: &str,
    cursor: &mut TreeCursor,
    acc: &mut Vec<TypePosInfo>,
) {
    let node = cursor.node();
    let type_string = node.kind();
    match type_string {
        // https://github.com/tree-sitter/tree-sitter-rust/blob/b77c0d8ac28a7c143224e6ed9b4f9e4bd044ff5b/src/node-types.json#L393-L464
        // "{" 같은것도 node 단위 취급되어서 제외해줘야한다.
        "block_comment" | "line_comment" | "{" | "}" | "(" | ")" | "<" | ">" | "//" | "/*"
        | "*/" | "[" | "]" | "=" => {}
        _ => {
            // pub fn block_on<Copy> 여기서 <Copy> 이런 것들을 찾아내는 것
            let start_byte = node.start_byte();
            let end_byte = node.end_byte();
            let start_point = node.start_position();
            let end_point = node.end_position();

            let _struct_name = node;
            // dbg!(&struct_name);

            // avoid unicode-byte index mismatch problem
            // - just ignore them
            let source_chars: Vec<char> = source_code.chars().collect();
            if source_chars.len() <= end_byte - 1 {
                return;
            }

            let type_info: TypePosInfo =
                (type_string, start_byte, end_byte, start_point, end_point);
            acc.push(type_info);
        }
    }
}

// 위에서 수정하며 만든 코드를 응용해, 독특한 변이 방법을 만든다.
// 그냥 뭐가 오건 deletion을 가능한 모든 node에 대해 적용한다.
pub fn mutate_delete_only(
    source_code: &str,
    structs: &Vec<TypePosInfo>,
    mutation_count: i32,
) -> Vec<String> {
    // Add more type mappings as needed

    let mut modified_versions = Vec::new();
    let mut index = 0;

    if mutation_count == 0 {
        for &(type_string, start_byte, end_byte, start_point, end_point) in structs.iter() {
            // start_byte와 end_byte를 이용해 원본 코드를 자른다.
            // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
            let before = &source_code[..start_byte];
            let after = &source_code[end_byte..];
            let original = &source_code[start_byte..end_byte];

            index += 1;
            // type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.

            modified_versions.push(format!("{}{}{}", before, "", after));
            // print difference between original and modified version
            // _name is not a code. find it with before and after.

            println!(
                "[{}] {}-{} {} : {} -> {}",
                index, start_point, end_point, type_string, original, ""
            );
        }
    }
    // mutation_count가 0이 아니면, mutation_count만큼만 변이를 만든다.
    // 이때 선택하는 변이는 랜덤.
    else {
        // 일단 structs 안에서 mutation_count 만큼의 랜덤한 선택이 필요하다.
        // structs.len()가 mutation_count보다 작으면, structs.len()만큼만 선택하면 된다.
        let sample: Vec<TypePosInfo> = structs
            .choose_multiple(&mut rand::thread_rng(), mutation_count as usize)
            .map(|&x| x)
            .collect();
        for &(type_string, start_byte, end_byte, start_point, end_point) in sample.iter() {
            // start_byte와 end_byte를 이용해 원본 코드를 자른다.
            // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
            let before = &source_code[..start_byte];
            let after = &source_code[end_byte..];
            let original = &source_code[start_byte..end_byte];

            index += 1;
            // type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.

            modified_versions.push(format!("{}{}{}", before, "", after));
            // print difference between original and modified version
            // _name is not a code. find it with before and after.

            println!(
                "[{}] {}-{} {} : {} -> {}",
                index, start_point, end_point, type_string, original, ""
            );
        }
    }

    //변형된 버전을 돌려줌.
    modified_versions
}

// 위에서 수정하며 만든 코드를 응용해, 독특한 변이 방법을 만든다.
// 자기 코드 안에서 타입을 찾아 전부 저장하고, 자기 자신이 지닌 같은 타입을 찾아 변이를 만든다.
// 이때 중복이 발생하면 안됨.
pub fn mutate_self(
    source_code: &str,
    structs: &Vec<TypePosInfo>,
    mutation_count: i32,
) -> Vec<String> {
    let mut new_exprs: HashMap<&str, Vec<&str>> = HashMap::new();
    // new_exprs.insert("type_identifier", vec!["", "i32", "str", "Copy"]);
    // new_exprs.insert("unit_type", vec!["", "f32", "char", "Clone", "a"]);
    // new_exprs.insert("identifier", vec!["", "f32", "char", "Clone", "a"]);

    let mut modified_versions = Vec::new();
    let mut index = 0;

    // type을 기준으로 HashMap에 코드를 추가. 이후에 타입을 찾아서 변이를 만들 때 사용.
    // HashMap에 일치하는 key가 없다면 새로 만들고, 있다면 내용물 vec에 추가한다.
    for &(type_string, start_byte, end_byte, _start_point, _end_point) in structs.iter() {
        // 여기서 sample을 얻고, type 정보를 기준으로 HashMap에 추가한다.
        // new_exprs.insert("identifier", vec!["", "f32", "char", "Clone", "a"]);
        // 위를 보면 알겠지만, HashMap new_expr는 value로 vec을 가진다.
        // 이 vec에 새로운 정보를 추가하는게 중요!

        // new_exprs.entry(type_string).or_insert(value);

        let sample = &source_code[start_byte..end_byte];
        if let Some(exprs) = new_exprs.get_mut(type_string) {
            if exprs.contains(&sample) {
                // 이미 있으면 추가하지 않는다. duplicate 방지
                continue;
            } else {
                exprs.push(sample);
            }
        } else {
            new_exprs.insert(type_string, vec!["", sample]);
        }
    }

    if mutation_count == 0 {
        for &(type_string, start_byte, end_byte, start_point, end_point) in structs.iter() {
            // start_byte와 end_byte를 이용해 원본 코드를 자른다.
            // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
            let before = &source_code[..start_byte];
            let after = &source_code[end_byte..];
            let original = &source_code[start_byte..end_byte];

            // type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.
            if let Some(exprs) = new_exprs.get(type_string) {
                for n in exprs {
                    // n.to_string() != 이 부분은 중복 방지를 위해 넣은 것. original code와 다른 것만 출력한다.
                    if n.to_string() != original && n.to_string() != "" {
                        modified_versions.push(format!("{}{}{}", before, n, after));
                        index += 1;
                        // print difference between original and modified version
                        println!(
                            "[{}] {}-{} {} : {} -> {}",
                            index, start_point, end_point, type_string, original, n
                        );
                    }
                }
            }
        }
    }
    // mutation_count가 0이 아니면, mutation_count만큼만 변이를 만든다.
    // 당연히 여기도 structs 안에서 랜덤한 선택이 필요.
    else {
        let sample: Vec<TypePosInfo> = structs
            .choose_multiple(&mut rand::thread_rng(), structs.len())
            .map(|&x| x)
            .collect();
        // 여긴 방법이 조금 다르다. 일단 뒤섞고, mutation_count만큼만 선택한다.
        // 방법은... 대충 하자 대충 그냥 랜덤선택
        let mut check_zero_mutation: i32 = 0;
        // mutation 자체가 발생하지 않는 경우, index가 증가하지 않는다.
        // zero_mutation이 100을 넘어가는동안 index가 증가하지 않으면, mutation이 발생하지 않는 것으로 간주한다.
        println!("start while");
        while index < mutation_count {
            check_zero_mutation += 1;
            if check_zero_mutation > 100 && index == 0 {
                break;
            }
            if let Some(selected) = sample.choose(&mut rand::thread_rng()) {
                // let selected = sample.choose(&mut rand::thread_rng()).unwrap();
                let &(type_string, start_byte, end_byte, start_point, end_point) = selected;
                // start_byte와 end_byte를 이용해 원본 코드를 자른다.
                // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
                let before = &source_code[..start_byte];
                let after = &source_code[end_byte..];
                let original = &source_code[start_byte..end_byte];

                // type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.
                if let Some(exprs) = new_exprs.get(type_string) {
                    // 여기도 마찬가지로, exprs 중 하나를 랜덤으로 선택해서 변이를 만든다.
                    let n = exprs.choose(&mut rand::thread_rng()).unwrap();
                    // n.to_string() != 이 부분은 중복 방지를 위해 넣은 것. original code와 다른 것만 출력한다.
                    if n.to_string() != original && n.to_string() != "" {
                        modified_versions.push(format!("{}{}{}", before, n, after));
                        index += 1;
                        // print difference between original and modified version
                        println!(
                            "[{}] {}-{} {} : {} -> {}",
                            index, start_point, end_point, type_string, original, n
                        );
                    }
                }
            }
            
        }
        println!("end while");
    }

    //변형된 버전을 돌려줌.
    modified_versions
}

pub fn mutate_splice(
    source_code: &str,
    new_exprssions: &HashMap<&str, Vec<String>>,
    structs: &Vec<TypePosInfo>,
    mutation_count: i32,
) -> Vec<String> {
    let mut modified_versions = Vec::new();
    let mut index = 0;

    if mutation_count == 0 {
        panic!("Stopped because there might be too much mutated files.");
    }
    // mutation_count가 0이 아니면, mutation_count만큼만 변이를 만든다.
    // 당연히 여기도 structs 안에서 랜덤한 선택이 필요.
    else {
        let sample: Vec<TypePosInfo> = structs
            .choose_multiple(&mut rand::thread_rng(), structs.len())
            .map(|&x| x)
            .collect();
        // 여긴 방법이 조금 다르다. 일단 뒤섞고, mutation_count만큼만 선택한다.
        // 방법은... 대충 하자 대충 그냥 랜덤선택
        let mut check_zero_mutation: i32 = 0;
        // mutation 자체가 발생하지 않는 경우, index가 증가하지 않는다.
        // zero_mutation이 100을 넘어가는동안 index가 증가하지 않으면, mutation이 발생하지 않는 것으로 간주한다.
        println!("start while");
        while index < mutation_count {
            check_zero_mutation += 1;
            if check_zero_mutation > 100 && index == 0 {
                break;
            }
            if let Some(selected) = sample.choose(&mut rand::thread_rng()) {
                // let selected = sample.choose(&mut rand::thread_rng()).unwrap();
                // let &(type_string, start_byte, end_byte, start_point, end_point) = selected;

                let &(type_string, start_byte, end_byte, start_point, end_point) = selected;
                // start_byte와 end_byte를 이용해 원본 코드를 자른다.
                // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
                let before = &source_code[..start_byte];
                let after = &source_code[end_byte..];
                let original = &source_code[start_byte..end_byte];

                // type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.
                if let Some(exprs) = new_exprssions.get(type_string) {
                    // 여기도 마찬가지로, exprs 중 하나를 랜덤으로 선택해서 변이를 만든다.
                    if let Some(n) = exprs.choose(&mut rand::thread_rng()) {
                        // n.to_string() != 이 부분은 중복 방지를 위해 넣은 것. original code와 다른 것만 출력한다.
                        if n.to_string() != original && n.to_string() != "" {
                            modified_versions.push(format!("{}{}{}", before, n, after));
                            index += 1;
                            // print difference between original and modified version
                            println!(
                                "[{}] {}-{} {} : {} -> {}",
                                index, start_point, end_point, type_string, original, n
                            );
                        }
                    }
                    /*
                    let n = exprs.choose(&mut rand::thread_rng()).unwrap();
                    // n.to_string() != 이 부분은 중복 방지를 위해 넣은 것. original code와 다른 것만 출력한다.
                    if n.to_string() != original && n.to_string() != "" {
                        modified_versions.push(format!("{}{}{}", before, n, after));
                        index += 1;
                        // print difference between original and modified version
                        println!(
                            "[{}] {}-{} {} : {} -> {}",
                            index, start_point, end_point, type_string, original, n
                        );
                    }
                    */
                }
            }
        }
        println!("end while");
    }

    //변형된 버전을 돌려줌.
    modified_versions
}

pub fn mutate_splice_randtype( // 이건 type이 다른 것도 넣어준다. 
    source_code: &str,
    new_exprssions: &HashMap<&str, Vec<String>>,
    structs: &Vec<TypePosInfo>,
    mutation_count: i32,
) -> Vec<String> {
    let mut modified_versions = Vec::new();
    let mut index = 0;

    if mutation_count == 0 {
        panic!("Stopped because there might be too much mutated files.");
    }
    // mutation_count가 0이 아니면, mutation_count만큼만 변이를 만든다.
    // 당연히 여기도 structs 안에서 랜덤한 선택이 필요.
    else {
        let keylist: Vec<&str> = new_exprssions.keys().map(|&x| x).collect();

        let sample: Vec<TypePosInfo> = structs
            .choose_multiple(&mut rand::thread_rng(), structs.len())
            .map(|&x| x)
            .collect();
        // 여긴 방법이 조금 다르다. 일단 뒤섞고, mutation_count만큼만 선택한다.
        // 방법은... 대충 하자 대충 그냥 랜덤선택
        let mut check_zero_mutation: i32 = 0;
        // mutation 자체가 발생하지 않는 경우, index가 증가하지 않는다.
        // zero_mutation이 100을 넘어가는동안 index가 증가하지 않으면, mutation이 발생하지 않는 것으로 간주한다.
        println!("start while");
        while index < mutation_count {
            check_zero_mutation += 1;
            if check_zero_mutation > 100 && index == 0 {
                break;
            }
            if let Some(selected) = sample.choose(&mut rand::thread_rng()) {
                // let selected = sample.choose(&mut rand::thread_rng()).unwrap();
                // let &(type_string, start_byte, end_byte, start_point, end_point) = selected;

                let &(type_string, start_byte, end_byte, start_point, end_point) = selected;
                // start_byte와 end_byte를 이용해 원본 코드를 자른다.
                // before는 바꾸고자 하는 코드 앞, after는 뒤, original은 바뀌는 부분의 원본 코드
                let before = &source_code[..start_byte];
                let after = &source_code[end_byte..];
                let original = &source_code[start_byte..end_byte];

                // 원래는 type_string에 해당하는 타입을 찾아서, 그 타입에 해당하는 변형된 버전을 찾아서 modified_versions에 넣어준다.
                // 하지만 이건 random한 type을 사용해야 하니, type_string을 랜덤으로 선택한다.
                let rand_type_string = keylist.choose(&mut rand::thread_rng()).unwrap();
                if let Some(exprs) = new_exprssions.get(rand_type_string) {
                    // 여기도 마찬가지로, exprs 중 하나를 랜덤으로 선택해서 변이를 만든다.
                    if let Some(n) = exprs.choose(&mut rand::thread_rng()) {
                        // n.to_string() != 이 부분은 중복 방지를 위해 넣은 것. original code와 다른 것만 출력한다.
                        if n.to_string() != original && n.to_string() != "" {
                            modified_versions.push(format!("{}{}{}", before, n, after));
                            index += 1;
                            // print difference between original and modified version
                            println!(
                                "[{}] {}-{} {}:{} -> {}:{}",
                                index, start_point, end_point, original, type_string, n, rand_type_string
                            );
                        }
                    }
                    /*
                    let n = exprs.choose(&mut rand::thread_rng()).unwrap();
                    // n.to_string() != 이 부분은 중복 방지를 위해 넣은 것. original code와 다른 것만 출력한다.
                    if n.to_string() != original && n.to_string() != "" {
                        modified_versions.push(format!("{}{}{}", before, n, after));
                        index += 1;
                        // print difference between original and modified version
                        println!(
                            "[{}] {}-{} {} : {} -> {}",
                            index, start_point, end_point, type_string, original, n
                        );
                    }
                    */
                }
            }
        }
        println!("end while");
    }

    //변형된 버전을 돌려줌.
    modified_versions
}

// 이 함수를 통해 source code를 받아서 struct을 수정한 source code를 반환한다.
// 코드를 주면, parser로 parse 한 다음, found_structs에 struct 정보를 넣어준다.
// 그 다음 찾은 struct 정보를 모아 modify_types를 통해 수정한 후 반환한다.
pub fn get_struct_crushed_sources(
    source_code: &str,
    mutation_mode: i32,
    mutation_count: i32,
) -> Vec<String> {
    // https://tree-sitter.github.io/tree-sitter/creating-parsers 여기 적힌 형태로 parsing 해준다.
    let mut parser = tree_sitter::Parser::new();
    // rust 언어를 기준으로 parsing해줌.
    let language = tree_sitter_rust::language();
    parser.set_language(&language).unwrap();

    let tree = parser.parse(&source_code, None).unwrap();
    let mut found_structs: Vec<TypePosInfo> = Vec::new();
    //tree가 복잡복잡하고 주어진 tree-sitter의 탐색 방법 제한이 커서, vertical과 horizontal로 나눠서 탐색한다.
    visit_vertical(&source_code, &mut tree.walk(), &mut found_structs);

    // modify_types_my(&source_code, &found_structs)
    // mutate_delete_only(&source_code, &found_structs)
    // 입력 옵션을 받아 deletion only 말고 다른것도 하게 만들자.
    if mutation_mode == 0 {
        return mutate_delete_only(&source_code, &found_structs, mutation_count);
    } else if mutation_mode == 1 {
        return mutate_self(&source_code, &found_structs, mutation_count);
    } else {
        panic!("No such mutation mode.");
    }
}

/*
pub fn get_splice_parts(source_code: String, found_structs: Vec<TypePosInfo>) -> Vec<TypePosInfo> {
    // https://tree-sitter.github.io/tree-sitter/creating-parsers 여기 적힌 형태로 parsing 해준다.
    let mut parser = tree_sitter::Parser::new();
    // rust 언어를 기준으로 parsing해줌.
    let language = tree_sitter_rust::language();
    parser.set_language(&language).unwrap();

    let tree = parser.parse(&source_code, None).unwrap();
    //tree가 복잡복잡하고 주어진 tree-sitter의 탐색 방법 제한이 커서, vertical과 horizontal로 나눠서 탐색한다.
    visit_vertical(&source_code, &mut tree.walk(), &mut found_structs);

    return found_structs;
}
 */

pub fn get_splice_parts(source_code: &String) -> Vec<TypePosInfo<'static>> {
    // https://tree-sitter.github.io/tree-sitter/creating-parsers 여기 적힌 형태로 parsing 해준다.
    let mut parser = tree_sitter::Parser::new();
    // rust 언어를 기준으로 parsing해줌.
    let language = tree_sitter_rust::language();
    parser.set_language(&language).unwrap();

    let tree = parser.parse(&source_code, None).unwrap();
    let mut found_structs: Vec<TypePosInfo<'static>> = Vec::new();
    //tree가 복잡복잡하고 주어진 tree-sitter의 탐색 방법 제한이 커서, vertical과 horizontal로 나눠서 탐색한다.
    visit_vertical(&source_code, &mut tree.walk(), &mut found_structs);

    return found_structs;
}

// use clap cli parser
#[derive(Parser, Debug)]
struct Cli {
    /// locate input directory path
    #[arg(short, long)]
    input_dir: Option<String>,
    /// locate output directory path
    #[arg(short, long)]
    output_dir: Option<String>,
    /// 0: deletion only, 1: self splice mutation, 2: all file splice mutation, 3: all file splice mutation with random type
    #[arg(short, long)]
    mode: i32,
    /// count of mutation for each seed file.
    /// if 0, then generate all possible mutation files
    #[arg(short, long)]
    file_count: Option<i32>,
}

pub fn main() {
    let args = Cli::parse();

    let output_dir: PathBuf = if let Some(o) = args.output_dir {
        // if directory exists then use it, otherwise create it (and notice it to the user)
        if !PathBuf::from(&o).exists() {
            fs::create_dir_all(&o).unwrap();
            println!("Created output directory: {}", o);
        }
        o.into()
    } else {
        // notice it uses current dir to user
        let current_dir = std::env::current_dir().unwrap();
        println!(
            "No output directory provided, using current directory: {:?}",
            current_dir
        );
        current_dir
    };

    if let Some(input_dir) = args.input_dir {
        let mutation_mode = args.mode;
        let mutation_count = args.file_count.unwrap_or(0);

        // 여긴 모든 파일로부터 mutation splice code를 얻어오게 시킨다.
        // 이걸 하려면 1. input_dir 내 모든 entry에 대해 mutate_self에 있던 new_exprs.insert를 실행해
        // 아주아주 거대한 new_exprs를 만든 다음
        // 2. mutate_self와 유사한 방법으로 각 파일을 mutate.
        if mutation_mode == 2 || mutation_mode == 3 {
            if mutation_count == 0 {
                panic!("Stopped because there might be too much mutated files.");
            }
            // 아래 통상적인 코드와 달리, 이 splice_mutates는 splice용 코드 뭉치를 따로 저장하고, 이거를 같이 보내준다.
            let mut new_expressions: HashMap<&str, Vec<String>> = HashMap::new();

            for entry in tqdm(WalkDir::new(&input_dir).into_iter()).style(tqdm::Style::Block) {
                let mut splice_mutates: Vec<TypePosInfo> = vec![];
                let entry = entry.unwrap();
                let path = entry.path();
                // file name을 저장한다.
                if let Some(ext) = path.extension() {
                    if path.is_file() && ext.to_string_lossy() == "rs" {
                        // dbg!(path);
                        let source_code = fs::read_to_string(path).unwrap();
                        if source_code.lines().count() < 500 { // 너무 큰 파일 안씀
                            // 이거 &source_code 도대체 왜 에러가...
                            // splice_mutates.append(&mut get_splice_parts(&source_code));
                            splice_mutates.append(&mut get_splice_parts(&source_code));
                        }
                    }
                }

                // type을 기준으로 HashMap에 코드를 추가. 이후에 타입을 찾아서 변이를 만들 때 사용.
                // HashMap에 일치하는 key가 없다면 새로 만들고, 있다면 내용물 vec에 추가한다.
                for &(type_string, start_byte, end_byte, _start_point, _end_point) in
                    splice_mutates.iter()
                {
                    // 여기서 source code를 변이할 하나만 건네주니, 범위도 내용도 안맞아서 망해버린다.
                    // 평소 하듯 아래 코드처럼 sample을 만들면 source code 범위 지정 과정에서 index out of bounds 발생
                    // 방법은 2가지다. source code를 TypePosInfo안에 넣어주거나, 아예 이 sample 자체를 외부에서 만들어서 전달하거나.
                    // 당연히 후자를 해야지..?
                    let source_code = fs::read_to_string(path).unwrap();
                        if source_code.lines().count() < 500 { // 너무 큰 파일 안씀
                        // 왜안되는건데 왜
                        // 넘겨주는 모든 것을 &str이 아닌 String으로 바꿔버렸다.
                        // HashMap<&str, Vec<String>> 이렇게 바꿨다.
                        let sample = &source_code[start_byte..end_byte];
                        if let Some(exprs) = new_expressions.get_mut(type_string) {
                            if exprs.contains(&sample.to_string()) {
                                // 이미 있으면 추가하지 않는다. duplicate 방지
                                continue;
                            } else {
                                exprs.push(sample.to_string());
                            }
                        } else {
                            new_expressions
                                .insert(type_string, vec!["".to_string(), sample.to_string()]);
                        }
                    }
                }
            }
            // 이제 진짜 파일 단위의 변이.
            // 위에서 input_dir를 소모해버림. 이대로는 안된다.
            // 그래서, 위쪽에서 input_dir를 가져갈 때 &를 붙여 잠깐 가져가게 했다.
            for entry in tqdm(WalkDir::new(input_dir).into_iter()).style(tqdm::Style::Block) {
                // 최종 결과를 담는게 mutated
                let mut mutated: Vec<String> = vec![];
                // 각 파일의 Vec<TypePosInfo>를 담는게 struct_per_file
                let mut struct_per_file: Vec<TypePosInfo> = vec![];
                let entry = entry.unwrap();
                let path = entry.path();
                // file name을 저장한다.
                if let Some(ext) = path.extension() {
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();
                    println!("filename : {}", name);
                    if path.is_file() && ext.to_string_lossy() == "rs" {
                        // dbg!(path);
                        // if source_code is larger then 500 lines, ignore it.
                        let source_code = fs::read_to_string(path).unwrap();
                        if source_code.lines().count() < 500 { // 너무 큰 파일 안씀
                            struct_per_file.append(&mut get_splice_parts(&source_code));
                            if mutation_mode == 3 {
                                mutated.append(&mut mutate_splice_randtype(
                                    &source_code,
                                    &new_expressions,
                                    &struct_per_file,
                                    mutation_count,
                                ));
                            } else { /* mutation_mode == 2 */
                                mutated.append(&mut mutate_splice(
                                    &source_code,
                                    &new_expressions,
                                    &struct_per_file,
                                    mutation_count,
                                ));
                            }
                        }
                    }
                }
                println!("testing2");
                for (idx, src) in mutated.iter().enumerate() {
                    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                    let file_name = format!("mut_{}_{}.rs", file_name, (idx + 1).to_string());
                    let file_path = output_dir.join(file_name);
                    fs::write(file_path, src).unwrap();
                }
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                println!(
                    "Number of generated files of {}: {}",
                    file_name,
                    mutated.len()
                );
            }
        }
        // 여긴 통상적으로 진행
        else {
            for entry in tqdm(WalkDir::new(input_dir).into_iter()).style(tqdm::Style::Block) {
                let mut mutated: Vec<String> = vec![];
                let entry = entry.unwrap();
                let path = entry.path();
                // file name을 저장한다.
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                if let Some(ext) = path.extension() {
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();
                    println!("filename : {}", name);
                    if path.is_file() && ext.to_string_lossy() == "rs" {
                        // dbg!(path);
                        let source_code = fs::read_to_string(path).unwrap();
                        if source_code.lines().count() < 500 { // 너무 큰 파일 안씀
                            println!("testing1");
                            mutated.append(&mut get_struct_crushed_sources(
                                &source_code,
                                mutation_mode,
                                mutation_count,
                            ));
                        }
                    }
                }
                println!("testing2");
                for (idx, src) in mutated.iter().enumerate() {
                    let file_name = format!("mut_{}_{}.rs", file_name, (idx + 1).to_string());
                    let file_path = output_dir.join(file_name);
                    fs::write(file_path, src).unwrap();
                }
                println!(
                    "Number of generated files of {}: {}",
                    file_name,
                    mutated.len()
                );
            }
        }
    } else {
        panic!("No input file or directory provided");
    };
}
