#[macro_use] extern crate criterion;
extern crate simplet2s;

use criterion::Criterion;
use simplet2s::convert;


fn criterion_benchmark(c: &mut Criterion) {
    convert("《第一批异体字整理表》已將「託」與「托」合併為「托」");

    c.bench_function("t2s naive", |b| {
        b.iter(|| {
            convert("《第一批异体字整理表》已將「託」與「托」合併為「托」");
        });
    });

    c.bench_function("t2s exclude", |b| {
        b.iter(|| {
            convert("「於」曾被《第一批異體字整理表》視為「于」的異體字廢除，後來恢復為規範字，但只用作姓氏人名，如樊於期，其他情況仍用「于」。");
        });
    });

    c.bench_function("t2s special convert type 1", |b| {
        b.iter(|| {    
            convert("「藉」其他意义仍然保留的，藉口、憑藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。");
        });
    });

    c.bench_function("t2s special convert type 2", |b| {
        b.iter(|| {
            convert("企畫 計畫 企劃 計劃 畫圖 畫畫");
        });
    });

    c.bench_function("t2s non convert", |b| {
        b.iter(|| {
            convert("英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成");
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
