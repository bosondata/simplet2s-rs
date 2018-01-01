extern crate phf;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet};

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

lazy_static! {
    // Traditional Chinese -> Not convert case
    static ref T2S_EXCLUDE: HashMap<&'static str, HashSet<&'static str>> = {
        hashmap!{
            "兒" => hashset!{"兒寬"},
            "覆" => hashset!{"答覆", "批覆", "回覆"},
            "夥" => hashset!{"甚夥"},
            "藉" => hashset!{"慰藉", "狼藉"},
            "瞭" => hashset!{"瞭望"},
            "麽" => hashset!{"幺麽"},
            "幺" => hashset!{"幺麽"},
            "於" => hashset!{"樊於"}
        }
    };
    // Traditional Chinese -> Special convert cases ( only convert in certain case )
    // not convert these chars if not in special cases
    static ref T2S_SPECIAL_CONVERT_TYPE_1: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        hashmap!{
            "藉" => hashmap!{"藉口" => "借", "憑藉" => "借"},
            "著" => hashmap!{"看著" => "着"},
            "苧" => hashmap!{"苧麻" => "苎"},
            "乾" => hashmap!{"乾燥" => "干", "乾爹" => "干", "餅乾" => "干", "乾枯" => "干", "乾旱" => "干"},
        }
    };
    // Traditional Chinese -> Special convert cases ( only convert in certain case )
    // convert these chars use naive mapping if not in special cases
    static ref T2S_SPECIAL_CONVERT_TYPE_2: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        hashmap!{
            "闔" => hashmap!{"闔家" => "合"},
            "鍾" => hashmap!{"鍾書" => "锺"},
            "讎" => hashmap!{"校讎" => "雠", "讎定" => "雠", "仇讎" => "雠"},
            "畫" => hashmap!{"計畫" => "划", "企畫" => "划"},
        }
    };
}

fn word_on_prefix<'a>(text: &'a str, char_indices: &'a [usize], index: usize) -> Option<&'a str> {
    let char_count = char_indices.len();
    if index + 1 < char_count {
        let byte_start = char_indices[index];
        let byte_end = if index + 2 < char_count {
            char_indices[index + 2]
        } else {
            text.len()
        };
        return unsafe { Some(text.get_unchecked(byte_start..byte_end)) };
    }
    None
}

fn word_on_suffix<'a>(text: &'a str, char_indices: &'a [usize], index: usize) -> Option<&'a str> {
    if index > 0 {
        let char_count = char_indices.len();
        let byte_start = char_indices[index - 1];
        let byte_end = if index + 1 < char_count {
            char_indices[index + 1]
        } else {
            text.len()
        };
        return unsafe { Some(text.get_unchecked(byte_start..byte_end)) };
    }
    None
}

/// Traditional Chinese to Simplified Chinese
/// convert some special cases according to 繁簡轉換一對多列表
/// on Wikipedia.
pub fn convert(text: &str) -> String {
    let mut ret = String::with_capacity(text.len());
    let char_indices: Vec<usize> = text.char_indices().map(|x| x.0).collect();
    let mut char_buf = [0; 4];
    for (index, char_) in text.chars().enumerate() {
        let str_: &str = char_.encode_utf8(&mut char_buf);
        if let Some(inner_set) = T2S_EXCLUDE.get(str_) {
            if let Some(prefix) = word_on_prefix(&text, &char_indices, index) {
                if inner_set.contains(&prefix[..]) {
                    ret.push_str(str_);
                    continue
                }
            }
            if let Some(suffix) = word_on_suffix(&text, &char_indices, index) {
                if inner_set.contains(&suffix[..]) {
                    ret.push_str(str_);
                    continue
                }
            }
        }
        if let Some(inner_map) = T2S_SPECIAL_CONVERT_TYPE_1.get(str_) {
            if let Some(prefix) = word_on_prefix(&text, &char_indices, index) {
                if let Some(val) = inner_map.get(&prefix[..]) {
                    ret.push_str(val);
                    continue;
                }
            }
            if let Some(suffix) = word_on_suffix(&text, &char_indices, index) {
                if let Some(val) = inner_map.get(&suffix[..]) {
                    ret.push_str(val);
                    continue;
                }
            }
            ret.push_str(str_);
            continue
        } else if let Some(inner_map) = T2S_SPECIAL_CONVERT_TYPE_2.get(str_) {
            if let Some(prefix) = word_on_prefix(&text, &char_indices, index) {
                if let Some(val) = inner_map.get(&prefix[..]) {
                    ret.push_str(val);
                    continue;
                }
            }
            if let Some(suffix) = word_on_suffix(&text, &char_indices, index) {
                if let Some(val) = inner_map.get(&suffix[..]) {
                    ret.push_str(val);
                    continue;
                }
            }
            ret.push_str(T2S_MAP.get(str_).unwrap_or(&str_));
            continue;
        }
        ret.push_str(T2S_MAP.get(str_).unwrap_or(&str_));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn test_t2s_naive() {
        assert_eq!(&convert("憂鬱的台灣烏龜"), "忧郁的台湾乌龟");
        assert_eq!(&convert("測試"), "测试");
        assert_eq!(
            &convert("《第一批异体字整理表》已將「託」與「托」合併為「托」"),
            "《第一批异体字整理表》已将「托」与「托」合并为「托」"
        );
        assert_eq!(&convert("宁是貯的本字，与寧没有关系"), "宁是贮的本字，与宁没有关系");
    }

    #[test]
    fn test_t2s_exclude() {
        assert_eq!(&convert("瞭读liǎo(瞭解)时，简作「了」"), "了读liǎo(了解)时，简作「了」");
        assert_eq!(&convert("西漢有御史大夫兒寬"), "西汉有御史大夫兒宽");
        assert_eq!(
            &convert("「於」曾被《第一批異體字整理表》視為「于」的異體字廢除，後來恢復為規範字，但只用作姓氏人名，如樊於期，其他情況仍用「于」。"),
            "「于」曾被《第一批异体字整理表》视为「于」的异体字废除，后来恢复为规范字，但只用作姓氏人名，如樊於期，其他情况仍用「于」。"
        );
        assert_eq!(
            &convert("麽」读mó(摩)时不简化，如「幺麽小丑」，读yāo(夭)的「么」作「幺」(「么」本字)"),
            "么」读mó(摩)时不简化，如「幺麽小丑」，读yāo(夭)的「么」作「幺」(「么」本字)"
        );
    }

    #[test]
    fn test_t2s_special_convert_type_1() {
        assert_eq!(
            &convert("「藉」其他意义仍然保留的，藉口、憑藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。"),
            "「藉」其他意义仍然保留的，借口、凭借的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。"
        );
        assert_eq!(&convert("而繁體字苧（zhù）是苧麻"), "而繁体字苧（zhù）是苎麻");
        assert_eq!(
            &convert("「乾」其他意义仍然保留的，乾坤、乾隆的乾不簡化为「干」，“乾燥”、“乾爹”的“乾”簡化為“干”"),
            "「乾」其他意义仍然保留的，乾坤、乾隆的乾不简化为「干」，“干燥”、“干爹”的“乾”简化为“干”"
        );
    }

    #[test]
    fn test_t2s_special_convert_type_2() {
        assert_eq!(&convert("作姓氏時「鍾」可以簡化為「锺"), "作姓氏时「钟」可以简化为「锺");
        assert_eq!(&convert("企畫 計畫 企劃 計劃 畫圖 畫畫"), "企划 计划 企划 计划 画图 画画");
    }
}
