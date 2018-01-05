#![feature(test)]
extern crate test;
extern crate simplet2s;

use test::Bencher;
use simplet2s::convert;


#[bench]
fn bench_t2s_naive(b: &mut Bencher) {
    convert("《第一批异体字整理表》已將「託」與「托」合併為「托」");
    b.iter(|| {
        convert("《第一批异体字整理表》已將「託」與「托」合併為「托」");
    });
}

#[bench]
fn bench_t2s_exclude(b: &mut Bencher) {
    convert("「於」曾被《第一批異體字整理表》視為「于」的異體字廢除，後來恢復為規範字，但只用作姓氏人名，如樊於期，其他情況仍用「于」。");
    b.iter(|| {
        convert("「於」曾被《第一批異體字整理表》視為「于」的異體字廢除，後來恢復為規範字，但只用作姓氏人名，如樊於期，其他情況仍用「于」。");
    });
}

#[bench]
fn bench_t2s_special_convert_type_1(b: &mut Bencher) {
    convert("「藉」其他意义仍然保留的，藉口、憑藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。");
    b.iter(|| {    
        convert("「藉」其他意义仍然保留的，藉口、憑藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。");
    });
}

#[bench]
fn bench_t2s_special_convert_type_2(b: &mut Bencher) {
    convert("企畫 計畫 企劃 計劃 畫圖 畫畫");
    b.iter(|| {
        convert("企畫 計畫 企劃 計劃 畫圖 畫畫");
    });
}

#[bench]
fn bench_t2s_non_convert(b: &mut Bencher) {
    convert("英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成");
    b.iter(|| {
        convert("英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成");
    });
}
