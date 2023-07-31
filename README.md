# `rustc-backtrace-filter`
`rustc-backtrace-filter` is a utility for filtering noisy, often irrelevant lines of out of 
backtraces when working on rustc.

rustc already supports `RUST_BACKTRACE=short` to cut out some lines around the query system, which
might be sufficient for your needs. However, there are still additional lines you might want to
filter out - like the compiler driver, or some of the panic machinery.

You can install `rustc-backtrace-filter` using Cargo:

```shell-session
$ cargo install --git https://github.com/davidtwco/rustc-backtrace-filter
$ rbf --version
```

`rustc-backtrace-filter` has a bunch of flags for configuring what you want stripped out of the
backtrace:

```shell-session
$ rbf --help
Simple utility to filter out unwanted parts of a rustc backtrace

Usage: rbf [OPTIONS]

Options:
  -o <OUTPUT>                        Path where filtered input should be output, defaults to stdout
  -d, --driver <DRIVER>              Should driver-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -f, --fn-trait <FN_TRAIT>          Should function-trait-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -p, --panics <PANICS>              Should panic/backtrace-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -q, --query-system <QUERY_SYSTEM>  Should query-system-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -t, --threads <THREADS>            Should thread-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -t, --timing <TIMING>              Should timing-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -t, --tls <TLS>                    Should tls-related lines be filtered from backtrace? [default: true] [possible values: true, false]
  -h, --help                         Print help
  -V, --version                      Print version
```

You can use it by piping rustc output into it:

```shell-session
$ rustc test.rs | rbf
```

Feel free to submit contributions to update the regexes used to filter out lines or to add new
groups of lines that you might want filtered out.

## Comparison
Here's an example of what `rustc-backtrace-filter` cuts out:

<details>
<summary>Full backtrace</summary>

```
error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:20
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                    ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:42
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                                          ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0601]: `main` function not found in crate `rpit_issue_113794`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:5:2
  |
5 | }
  |  ^ consider adding a `main` function to `tests/ui/impl-trait/rpit-issue-113794.rs`

error: internal compiler error: compiler/rustc_middle/src/ty/mod.rs:2273:13: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: TypeNs("Foo"), disambiguator: 0 }, DisambiguatedDefPathData { data: ImplTraitAssocTy, disambiguator: 0 }], krate: crate0 }

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/rust2/compiler/rustc_errors/src/lib.rs:1645:9
stack backtrace:
   0:        0x104e7aef4 - std::backtrace_rs::backtrace::libunwind::trace::h3cb1b6d3f64060a2
                               at /rustc/rust2/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x104e7aef4 - std::backtrace_rs::backtrace::trace_unsynchronized::h5c1123bc7df06fbb
                               at /rustc/rust2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x104e61f50 - std::sys_common::backtrace::_print_fmt::h4bfc4e6d28af462f
                               at /rustc/rust2/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x104e61f50 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7ec7d44f5800b6fb
                               at /rustc/rust2/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x104eef388 - core::fmt::rt::Argument::fmt::hc24877fdb6975be0
                               at /rustc/rust2/library/core/src/fmt/rt.rs:138:9
   5:        0x104eef388 - core::fmt::write::he1cb38b10ca96be2
                               at /rustc/rust2/library/core/src/fmt/mod.rs:1094:21
   6:        0x104e70d94 - std::io::Write::write_fmt::h97ffa9b10a7ffd87
                               at /rustc/rust2/library/std/src/io/mod.rs:1714:15
   7:        0x104e61e14 - std::sys_common::backtrace::_print::hac1ec3ffa99987c3
                               at /rustc/rust2/library/std/src/sys_common/backtrace.rs:47:5
   8:        0x104e61e14 - std::sys_common::backtrace::print::h9033ed90c58a90f1
                               at /rustc/rust2/library/std/src/sys_common/backtrace.rs:34:9
   9:        0x104e95af4 - std::panicking::panic_hook_with_disk_dump::{{closure}}::head56c566b95bea1
  10:        0x104e958c0 - std::panicking::panic_hook_with_disk_dump::hbdf7c3842ff1505a
                               at /rustc/rust2/library/std/src/panicking.rs:312:9
  11:        0x1133d0024 - rustc_driver_impl[ca8e74233664d98e]::install_ice_hook::{closure#0}
                               at /rustc/rust2/compiler/rustc_driver_impl/src/lib.rs:1386:13
  12:        0x104e96064 - std::panicking::rust_panic_with_hook::h75384d9d82b2267a
                               at /rustc/rust2/library/std/src/panicking.rs:733:13
  13:        0x1178fbae0 - std[1145156680bbdca3]::panicking::begin_panic::<rustc_errors[501e82f9faddf706]::ExplicitBug>::{closure#0}
                               at /rustc/rust2/library/std/src/panicking.rs:650:9
  14:        0x1178fba3c - std[1145156680bbdca3]::sys_common::backtrace::__rust_end_short_backtrace::<std[1145156680bbdca3]::panicking::begin_panic<rustc_errors[501e82f9faddf706]::ExplicitBug>::{closure#0}, !>
                               at /rustc/rust2/library/std/src/sys_common/backtrace.rs:151:18
  15:        0x118030fa8 - std[1145156680bbdca3]::panicking::begin_panic::<rustc_errors[501e82f9faddf706]::ExplicitBug>
                               at /rustc/rust2/library/std/src/panicking.rs:649:12
  16:        0x1177ac07c - std[1145156680bbdca3]::panic::panic_any::<rustc_errors[501e82f9faddf706]::ExplicitBug>
                               at /rustc/rust2/library/std/src/panic.rs:63:5
  17:        0x1177abff8 - <rustc_errors[501e82f9faddf706]::HandlerInner>::bug::<alloc[5ca83fcf9f22257b]::string::String>
                               at /rustc/rust2/compiler/rustc_errors/src/lib.rs:1645:9
  18:        0x1177abe60 - <rustc_errors[501e82f9faddf706]::Handler>::bug::<alloc[5ca83fcf9f22257b]::string::String>
                               at /rustc/rust2/compiler/rustc_errors/src/lib.rs:1058:9
  19:        0x1178b5924 - rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt::<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/util/bug.rs:35:34
  20:        0x1178b5848 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_opt::<rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>::{closure#0}, !>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:154:36
  21:        0x1178b57e8 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context_opt::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_opt<rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  22:        0x1178b5814 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_opt::<rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>::{closure#0}, !>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:154:5
  23:        0x1178b5880 - rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt::<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>
                               at /rustc/rust2/compiler/rustc_middle/src/util/bug.rs:31:5
  24:        0x118033198 - rustc_middle[ac34d1cf85a491af]::util::bug::bug_fmt
                               at /rustc/rust2/compiler/rustc_middle/src/util/bug.rs:15:5
  25:        0x1178ab130 - <rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::item_name::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/mod.rs:2273:13
  26:        0x1178ab130 - <core[7719a01daa12b007]::option::Option<rustc_span[b36c64ad6b7f09a2]::symbol::Symbol>>::unwrap_or_else::<<rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::item_name::{closure#0}>
                               at /rustc/rust2/library/core/src/option.rs:979:21
  27:        0x1178ab130 - <rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::item_name
                               at /rustc/rust2/compiler/rustc_middle/src/ty/mod.rs:2272:9
  28:        0x1175c845c - <rustc_infer[354a95a148735087]::infer::error_reporting::TypeErrCtxt>::report_concrete_failure
                               at /rustc/rust2/compiler/rustc_infer/src/infer/error_reporting/note.rs:247:33
  29:        0x1175ce81c - <rustc_infer[354a95a148735087]::infer::error_reporting::TypeErrCtxt>::report_region_errors
                               at /rustc/rust2/compiler/rustc_infer/src/infer/error_reporting/mod.rs:459:29
  30:        0x11737afe0 - <rustc_trait_selection[292d5665640db603]::traits::engine::ObligationCtxt>::resolve_regions_and_report_errors
                               at /rustc/rust2/compiler/rustc_trait_selection/src/traits/engine.rs:196:17
  31:        0x115d18890 - rustc_hir_analysis[20d141ca3328fe63]::check::compare_impl_item::check_type_bounds
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/check/compare_impl_item.rs:2172:5
  32:        0x115c9fcb4 - rustc_hir_analysis[20d141ca3328fe63]::check::check::check_item_type
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/check/check.rs:717:75
  33:        0x115ca56fc - rustc_hir_analysis[20d141ca3328fe63]::check::check::check_mod_item_types
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/check/check.rs:1559:9
  34:        0x116b2b2d4 - rustc_query_impl[3c31ea5222527dd1]::query_impl::check_mod_item_types::dynamic_query::{closure#2}::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:582:47
  35:        0x116b2b2d4 - rustc_query_impl[3c31ea5222527dd1]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[3c31ea5222527dd1]::query_impl::check_mod_item_types::dynamic_query::{closure#2}::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:506:18
  36:        0x116a24b6c - rustc_query_impl[3c31ea5222527dd1]::query_impl::check_mod_item_types::dynamic_query::{closure#2}
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:578:25
  37:        0x116a24b6c - <rustc_query_impl[3c31ea5222527dd1]::query_impl::check_mod_item_types::dynamic_query::{closure#2} as core[7719a01daa12b007]::ops::function::FnOnce<(rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt, rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId)>>::call_once
                               at /rustc/rust2/library/core/src/ops/function.rs:250:5
  38:        0x116ba6e04 - <rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false> as rustc_query_system[3b25ed4be998705f]::query::config::QueryConfig<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>>::compute
                               at /rustc/rust2/compiler/rustc_query_impl/src/lib.rs:116:9
  39:        0x116ba6e04 - rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:481:72
  40:        0x116ba6e04 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context::<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  41:        0x116ba6e04 - <std[1145156680bbdca3]::thread::local::LocalKey<core[7719a01daa12b007]::cell::Cell<*const ()>>>::try_with::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/library/std/src/thread/local.rs:270:16
  42:        0x116ba6e04 - <std[1145156680bbdca3]::thread::local::LocalKey<core[7719a01daa12b007]::cell::Cell<*const ()>>>::with::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/library/std/src/thread/local.rs:246:9
  43:        0x116a99a1c - rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context::<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:79:5
  44:        0x116a99a1c - <rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:149:13
  45:        0x116a99a1c - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context::<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:133:9
  46:        0x116a99a1c - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:111:36
  47:        0x116a99a1c - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context_opt::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  48:        0x116a99a1c - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:111:5
  49:        0x116a99a1c - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context::<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:124:5
  50:        0x116a99a1c - <rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:134:9
  51:        0x116a99a1c - rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:481:18
  52:        0x116a99a1c - rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt, false>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:414:9
  53:        0x116a99a1c - rustc_query_system[3b25ed4be998705f]::query::plumbing::try_execute_query::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt, false>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:359:13
  54:        0x1169ce684 - rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:811:32
  55:        0x1169ce684 - stacker[c9864c21b7be4a8a]::maybe_grow::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>
                               at /Users/davidtw/.cargo/registry/src/index.crates.io-6f17d22bba15001f/stacker-0.1.15/src/lib.rs:55:9
  56:        0x1169ce684 - rustc_data_structures[56d98c77cc95b6f9]::stack::ensure_sufficient_stack::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>
                               at /rustc/rust2/compiler/rustc_data_structures/src/stack.rs:17:5
  57:        0x1169ce684 - rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:811:5
  58:        0x1169ce684 - rustc_query_impl[3c31ea5222527dd1]::query_impl::check_mod_item_types::get_query_non_incr::__rust_end_short_backtrace
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:556:26
  59:        0x115c13530 - rustc_middle[ac34d1cf85a491af]::query::plumbing::query_ensure::<rustc_query_system[3b25ed4be998705f]::query::caches::VecCache<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 0usize]>>>
                               at /rustc/rust2/compiler/rustc_middle/src/query/plumbing.rs:171:9
  60:        0x115c13100 - <rustc_middle[ac34d1cf85a491af]::query::plumbing::TyCtxtEnsure>::check_mod_item_types::<rustc_span[b36c64ad6b7f09a2]::def_id::LocalDefId>
                               at /rustc/rust2/compiler/rustc_middle/src/query/plumbing.rs:346:17
  61:        0x115c13100 - rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}::{closure#0}
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/lib.rs:237:44
  62:        0x115c13100 - <rustc_middle[ac34d1cf85a491af]::hir::map::Map>::for_each_module::<rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}::{closure#0}>
                               at /rustc/rust2/compiler/rustc_middle/src/hir/map/mod.rs:622:13
  63:        0x115d8c3f0 - rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/lib.rs:237:9
  64:        0x115d8c3f0 - <rustc_data_structures[56d98c77cc95b6f9]::profiling::VerboseTimingGuard>::run::<(), rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}>
                               at /rustc/rust2/compiler/rustc_data_structures/src/profiling.rs:752:9
  65:        0x115d8c3f0 - <rustc_session[f7ad2173cb009370]::session::Session>::time::<(), rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}>
                               at /rustc/rust2/compiler/rustc_session/src/utils.rs:12:9
  66:        0x115cd5a1c - rustc_hir_analysis[20d141ca3328fe63]::check_crate
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/lib.rs:236:5
  67:        0x1135b5db0 - rustc_interface[8bdfa1e99f215169]::passes::analysis
                               at /rustc/rust2/compiler/rustc_interface/src/passes.rs:788:5
  68:        0x116b385bc - rustc_query_impl[3c31ea5222527dd1]::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:582:47
  69:        0x116b385bc - rustc_query_impl[3c31ea5222527dd1]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[3c31ea5222527dd1]::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:506:18
  70:        0x116be32e4 - rustc_query_impl[3c31ea5222527dd1]::query_impl::analysis::dynamic_query::{closure#2}
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:578:25
  71:        0x116be32e4 - <rustc_query_impl[3c31ea5222527dd1]::query_impl::analysis::dynamic_query::{closure#2} as core[7719a01daa12b007]::ops::function::FnOnce<(rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt, ())>>::call_once
                               at /rustc/rust2/library/core/src/ops/function.rs:250:5
  72:        0x116ba3528 - <rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false> as rustc_query_system[3b25ed4be998705f]::query::config::QueryConfig<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>>::compute
                               at /rustc/rust2/compiler/rustc_query_impl/src/lib.rs:116:9
  73:        0x116ba3528 - rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:481:72
  74:        0x116ba3528 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context::<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  75:        0x116ba3528 - <std[1145156680bbdca3]::thread::local::LocalKey<core[7719a01daa12b007]::cell::Cell<*const ()>>>::try_with::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/library/std/src/thread/local.rs:270:16
  76:        0x116ba3528 - <std[1145156680bbdca3]::thread::local::LocalKey<core[7719a01daa12b007]::cell::Cell<*const ()>>>::with::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/library/std/src/thread/local.rs:246:9
  77:        0x116a659fc - rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context::<rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:79:5
  78:        0x116a659fc - <rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:149:13
  79:        0x116a659fc - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context::<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:133:9
  80:        0x116a659fc - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:111:36
  81:        0x116a659fc - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context_opt::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:100:18
  82:        0x116a659fc - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_context::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:111:5
  83:        0x116a659fc - rustc_middle[ac34d1cf85a491af]::ty::context::tls::with_related_context::<<rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>::{closure#0}, rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:124:5
  84:        0x116a659fc - <rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt as rustc_query_system[3b25ed4be998705f]::query::QueryContext>::start_query::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:134:9
  85:        0x116a659fc - rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:481:18
  86:        0x116a659fc - rustc_query_system[3b25ed4be998705f]::query::plumbing::execute_job::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt, false>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:414:9
  87:        0x116a659fc - rustc_query_system[3b25ed4be998705f]::query::plumbing::try_execute_query::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt, false>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:359:13
  88:        0x1169d2c90 - rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:811:32
  89:        0x1169d2c90 - stacker[c9864c21b7be4a8a]::maybe_grow::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>
                               at /Users/davidtw/.cargo/registry/src/index.crates.io-6f17d22bba15001f/stacker-0.1.15/src/lib.rs:55:9
  90:        0x1169d2c90 - rustc_data_structures[56d98c77cc95b6f9]::stack::ensure_sufficient_stack::<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>::{closure#0}>
                               at /rustc/rust2/compiler/rustc_data_structures/src/stack.rs:17:5
  91:        0x1169d2c90 - rustc_query_system[3b25ed4be998705f]::query::plumbing::get_query_non_incr::<rustc_query_impl[3c31ea5222527dd1]::DynamicConfig<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[3c31ea5222527dd1]::plumbing::QueryCtxt>
                               at /rustc/rust2/compiler/rustc_query_system/src/query/plumbing.rs:811:5
  92:        0x1169d2c90 - rustc_query_impl[3c31ea5222527dd1]::query_impl::analysis::get_query_non_incr::__rust_end_short_backtrace
                               at /rustc/rust2/compiler/rustc_query_impl/src/plumbing.rs:556:26
  93:        0x1134328d8 - rustc_middle[ac34d1cf85a491af]::query::plumbing::query_get_at::<rustc_query_system[3b25ed4be998705f]::query::caches::SingleCache<rustc_middle[ac34d1cf85a491af]::query::erase::Erased<[u8; 1usize]>>>
                               at /rustc/rust2/compiler/rustc_middle/src/query/plumbing.rs:155:17
  94:        0x1134328d8 - <rustc_middle[ac34d1cf85a491af]::query::plumbing::TyCtxtAt>::analysis
                               at /rustc/rust2/compiler/rustc_middle/src/query/plumbing.rs:385:31
  95:        0x1134328d8 - <rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::analysis
                               at /rustc/rust2/compiler/rustc_middle/src/query/plumbing.rs:376:17
  96:        0x1134328d8 - rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}
                               at /rustc/rust2/compiler/rustc_driver_impl/src/lib.rs:444:48
  97:        0x1134328d8 - <rustc_middle[ac34d1cf85a491af]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context.rs:589:37
  98:        0x1134328d8 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context::<<rustc_middle[ac34d1cf85a491af]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:82:9
  99:        0x1134328d8 - <std[1145156680bbdca3]::thread::local::LocalKey<core[7719a01daa12b007]::cell::Cell<*const ()>>>::try_with::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context<<rustc_middle[ac34d1cf85a491af]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/library/std/src/thread/local.rs:270:16
 100:        0x1134328d8 - <std[1145156680bbdca3]::thread::local::LocalKey<core[7719a01daa12b007]::cell::Cell<*const ()>>>::with::<rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context<<rustc_middle[ac34d1cf85a491af]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/library/std/src/thread/local.rs:246:9
 101:        0x11343fc68 - rustc_middle[ac34d1cf85a491af]::ty::context::tls::enter_context::<<rustc_middle[ac34d1cf85a491af]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context/tls.rs:79:5
 102:        0x11343fc68 - <rustc_middle[ac34d1cf85a491af]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/compiler/rustc_middle/src/ty/context.rs:589:9
 103:        0x113442f30 - rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}
                               at /rustc/rust2/compiler/rustc_driver_impl/src/lib.rs:444:13
 104:        0x113442f30 - <rustc_interface[8bdfa1e99f215169]::interface::Compiler>::enter::<rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}::{closure#2}, core[7719a01daa12b007]::result::Result<core[7719a01daa12b007]::option::Option<rustc_interface[8bdfa1e99f215169]::queries::Linker>, rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/compiler/rustc_interface/src/queries.rs:389:19
 105:        0x1133bd86c - rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}
                               at /rustc/rust2/compiler/rustc_driver_impl/src/lib.rs:385:22
 106:        0x1133bd86c - rustc_interface[8bdfa1e99f215169]::interface::run_compiler::<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /rustc/rust2/compiler/rustc_interface/src/interface.rs:339:21
 107:        0x1133bd86c - rustc_span[b36c64ad6b7f09a2]::set_source_map::<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /rustc/rust2/compiler/rustc_span/src/lib.rs:1041:5
 108:        0x1133bbd4c - rustc_interface[8bdfa1e99f215169]::interface::run_compiler::<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}
                               at /rustc/rust2/compiler/rustc_interface/src/interface.rs:333:13
 109:        0x1133bbd4c - <scoped_tls[b26d5237952552e8]::ScopedKey<rustc_span[b36c64ad6b7f09a2]::SessionGlobals>>::set::<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /Users/davidtw/.cargo/registry/src/index.crates.io-6f17d22bba15001f/scoped-tls-1.0.1/src/lib.rs:137:9
 110:        0x1133bb7f0 - rustc_span[b36c64ad6b7f09a2]::create_session_globals_then::<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}>
                               at /rustc/rust2/compiler/rustc_span/src/lib.rs:120:5
 111:        0x1133bb7f0 - rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals::<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /rustc/rust2/compiler/rustc_interface/src/util.rs:163:38
 112:        0x1133bb7f0 - std[1145156680bbdca3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/library/std/src/sys_common/backtrace.rs:135:18
 113:        0x1133c4078 - <std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_::<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /rustc/rust2/library/std/src/thread/mod.rs:529:17
 114:        0x1133c4078 - <core[7719a01daa12b007]::panic::unwind_safe::AssertUnwindSafe<<std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[7719a01daa12b007]::ops::function::FnOnce<()>>::call_once
                               at /rustc/rust2/library/core/src/panic/unwind_safe.rs:271:9
 115:        0x1133c8d4c - std[1145156680bbdca3]::panicking::try::do_call::<core[7719a01daa12b007]::panic::unwind_safe::AssertUnwindSafe<<std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/library/std/src/panicking.rs:524:40
 116:        0x1133c8d4c - std[1145156680bbdca3]::panicking::try::<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, core[7719a01daa12b007]::panic::unwind_safe::AssertUnwindSafe<<std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /rustc/rust2/library/std/src/panicking.rs:488:19
 117:        0x1133c553c - std[1145156680bbdca3]::panic::catch_unwind::<core[7719a01daa12b007]::panic::unwind_safe::AssertUnwindSafe<<std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>
                               at /rustc/rust2/library/std/src/panic.rs:142:14
 118:        0x1133c553c - <std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_::<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1}
                               at /rustc/rust2/library/std/src/thread/mod.rs:528:30
 119:        0x1133c553c - <<std[1145156680bbdca3]::thread::Builder>::spawn_unchecked_<rustc_interface[8bdfa1e99f215169]::util::run_in_thread_pool_with_globals<rustc_interface[8bdfa1e99f215169]::interface::run_compiler<core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>, rustc_driver_impl[ca8e74233664d98e]::run_compiler::{closure#1}>::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7719a01daa12b007]::result::Result<(), rustc_span[b36c64ad6b7f09a2]::ErrorGuaranteed>>::{closure#1} as core[7719a01daa12b007]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /rustc/rust2/library/core/src/ops/function.rs:250:5
 120:        0x104e52544 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1f39e1de09ea5441
                               at /rustc/rust2/library/alloc/src/boxed.rs:2007:9
 121:        0x104e52544 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0437a3930894c629
                               at /rustc/rust2/library/alloc/src/boxed.rs:2007:9
 122:        0x104e58cd8 - std::sys::unix::thread::Thread::new::thread_start::h6180736b9df30b38
                               at /rustc/rust2/library/std/src/sys/unix/thread.rs:108:17
 123:        0x184c27034 - __pthread_joiner_wake

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.73.0-dev running on aarch64-apple-darwin

query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0261, E0601.
For more information about an error, try `rustc --explain E0261`.
```

</details>

<details>
<summary>Short backtrace</summary>

```
error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:20
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                    ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:42
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                                          ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0601]: `main` function not found in crate `rpit_issue_113794`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:5:2
  |
5 | }
  |  ^ consider adding a `main` function to `tests/ui/impl-trait/rpit-issue-113794.rs`

error: internal compiler error: compiler/rustc_middle/src/ty/mod.rs:2273:13: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: TypeNs("Foo"), disambiguator: 0 }, DisambiguatedDefPathData { data: ImplTraitAssocTy, disambiguator: 0 }], krate: crate0 }

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/rust2/compiler/rustc_errors/src/lib.rs:1645:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
             at ./library/std/src/panicking.rs:649:12
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
             at ./library/std/src/panic.rs:63:5
   2: <rustc_errors::HandlerInner>::bug::<alloc::string::String>
             at ./compiler/rustc_errors/src/lib.rs:1645:9
   3: <rustc_errors::Handler>::bug::<alloc::string::String>
             at ./compiler/rustc_errors/src/lib.rs:1058:9
   4: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>::{closure#0}
             at ./compiler/rustc_middle/src/util/bug.rs:35:34
   5: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, !>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context/tls.rs:154:36
   6: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_opt<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
             at ./compiler/rustc_middle/src/ty/context/tls.rs:100:18
   7: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, !>
             at ./compiler/rustc_middle/src/ty/context/tls.rs:154:5
   8: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
             at ./compiler/rustc_middle/src/util/bug.rs:31:5
   9: rustc_middle::util::bug::bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:15:5
  10: <rustc_middle::ty::context::TyCtxt>::item_name::{closure#0}
             at ./compiler/rustc_middle/src/ty/mod.rs:2273:13
  11: <core::option::Option<rustc_span::symbol::Symbol>>::unwrap_or_else::<<rustc_middle::ty::context::TyCtxt>::item_name::{closure#0}>
             at ./library/core/src/option.rs:979:21
  12: <rustc_middle::ty::context::TyCtxt>::item_name
             at ./compiler/rustc_middle/src/ty/mod.rs:2272:9
  13: <rustc_infer::infer::error_reporting::TypeErrCtxt>::report_concrete_failure
             at ./compiler/rustc_infer/src/infer/error_reporting/note.rs:247:33
  14: <rustc_infer::infer::error_reporting::TypeErrCtxt>::report_region_errors
             at ./compiler/rustc_infer/src/infer/error_reporting/mod.rs:459:29
  15: <rustc_trait_selection::traits::engine::ObligationCtxt>::resolve_regions_and_report_errors
             at ./compiler/rustc_trait_selection/src/traits/engine.rs:196:17
  16: rustc_hir_analysis::check::compare_impl_item::check_type_bounds
             at ./compiler/rustc_hir_analysis/src/check/compare_impl_item.rs:2172:5
  17: rustc_hir_analysis::check::check::check_item_type
             at ./compiler/rustc_hir_analysis/src/check/check.rs:717:75
  18: rustc_hir_analysis::check::check::check_mod_item_types
             at ./compiler/rustc_hir_analysis/src/check/check.rs:1559:9
  19: rustc_query_impl::query_impl::check_mod_item_types::dynamic_query::{closure#2}::{closure#0}
             at ./compiler/rustc_query_impl/src/plumbing.rs:582:47
  20: rustc_middle::query::plumbing::query_ensure::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 0]>>>
             at ./compiler/rustc_middle/src/query/plumbing.rs:171:9
  21: <rustc_middle::query::plumbing::TyCtxtEnsure>::check_mod_item_types::<rustc_span::def_id::LocalDefId>
             at ./compiler/rustc_middle/src/query/plumbing.rs:346:17
  22: rustc_hir_analysis::check_crate::{closure#6}::{closure#0}
             at ./compiler/rustc_hir_analysis/src/lib.rs:237:44
  23: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#6}::{closure#0}>
             at ./compiler/rustc_middle/src/hir/map/mod.rs:622:13
  24: rustc_hir_analysis::check_crate::{closure#6}
             at ./compiler/rustc_hir_analysis/src/lib.rs:237:9
  25: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_hir_analysis::check_crate::{closure#6}>
             at ./compiler/rustc_data_structures/src/profiling.rs:752:9
  26: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#6}>
             at ./compiler/rustc_session/src/utils.rs:12:9
  27: rustc_hir_analysis::check_crate
             at ./compiler/rustc_hir_analysis/src/lib.rs:236:5
  28: rustc_interface::passes::analysis
             at ./compiler/rustc_interface/src/passes.rs:788:5
  29: rustc_query_impl::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}
             at ./compiler/rustc_query_impl/src/plumbing.rs:582:47
  30: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
             at ./compiler/rustc_middle/src/query/plumbing.rs:155:17
  31: <rustc_middle::query::plumbing::TyCtxtAt>::analysis
             at ./compiler/rustc_middle/src/query/plumbing.rs:385:31
  32: <rustc_middle::ty::context::TyCtxt>::analysis
             at ./compiler/rustc_middle/src/query/plumbing.rs:376:17
  33: rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}
             at ./compiler/rustc_driver_impl/src/lib.rs:444:48
  34: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context.rs:589:37
  35: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context/tls.rs:82:9
  36: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::try_with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             at ./library/std/src/thread/local.rs:270:16
  37: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             at ./library/std/src/thread/local.rs:246:9
  38: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             at ./compiler/rustc_middle/src/ty/context/tls.rs:79:5
  39: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             at ./compiler/rustc_middle/src/ty/context.rs:589:9
  40: rustc_driver_impl::run_compiler::{closure#1}::{closure#2}
             at ./compiler/rustc_driver_impl/src/lib.rs:444:13
  41: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             at ./compiler/rustc_interface/src/queries.rs:389:19
  42: rustc_driver_impl::run_compiler::{closure#1}
             at ./compiler/rustc_driver_impl/src/lib.rs:385:22
  43: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}
             at ./compiler/rustc_interface/src/interface.rs:339:21
  44: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             at ./compiler/rustc_span/src/lib.rs:1041:5
  45: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}
             at ./compiler/rustc_interface/src/interface.rs:333:13
  46: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             at /Users/davidtw/.cargo/registry/src/index.crates.io-6f17d22bba15001f/scoped-tls-1.0.1/src/lib.rs:137:9
  47: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}>
             at ./compiler/rustc_span/src/lib.rs:120:5
  48: rustc_interface::util::run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:163:38
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.73.0-dev running on aarch64-apple-darwin

query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0261, E0601.
For more information about an error, try `rustc --explain E0261`.
```

</details>

<details>
<summary>Full backtrace (filtered with <code>rustc-backtrace-filter</code>)</summary>

```
error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:20
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                    ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:42
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                                          ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0601]: `main` function not found in crate `rpit_issue_113794`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:5:2
  |
5 | }
  |  ^ consider adding a `main` function to `tests/ui/impl-trait/rpit-issue-113794.rs`

error: internal compiler error: compiler/rustc_middle/src/ty/mod.rs:2273:13: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: TypeNs("Foo"), disambiguator: 0 }, DisambiguatedDefPathData { data: ImplTraitAssocTy, disambiguator: 0 }], krate: crate0 }

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/rust2/compiler/rustc_errors/src/lib.rs:1645:9
stack backtrace:
  17:        0x114147ff8 - <rustc_errors[501e82f9faddf706]::HandlerInner>::bug::<alloc[5ca83fcf9f22257b]::string::String>
                               at /rustc/rust2/compiler/rustc_errors/src/lib.rs:1645:9
  18:        0x114147e60 - <rustc_errors[501e82f9faddf706]::Handler>::bug::<alloc[5ca83fcf9f22257b]::string::String>
                               at /rustc/rust2/compiler/rustc_errors/src/lib.rs:1058:9
  19:        0x114251924 - rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt::<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/util/bug.rs:35:34
  23:        0x114251880 - rustc_middle[ac34d1cf85a491af]::util::bug::opt_span_bug_fmt::<rustc_span[b36c64ad6b7f09a2]::span_encoding::Span>
                               at /rustc/rust2/compiler/rustc_middle/src/util/bug.rs:31:5
  24:        0x1149cf198 - rustc_middle[ac34d1cf85a491af]::util::bug::bug_fmt
                               at /rustc/rust2/compiler/rustc_middle/src/util/bug.rs:15:5
  25:        0x114247130 - <rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::item_name::{closure#0}
                               at /rustc/rust2/compiler/rustc_middle/src/ty/mod.rs:2273:13
  26:        0x114247130 - <core[7719a01daa12b007]::option::Option<rustc_span[b36c64ad6b7f09a2]::symbol::Symbol>>::unwrap_or_else::<<rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::item_name::{closure#0}>
                               at /rustc/rust2/library/core/src/option.rs:979:21
  27:        0x114247130 - <rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::item_name
                               at /rustc/rust2/compiler/rustc_middle/src/ty/mod.rs:2272:9
  28:        0x113f6445c - <rustc_infer[354a95a148735087]::infer::error_reporting::TypeErrCtxt>::report_concrete_failure
                               at /rustc/rust2/compiler/rustc_infer/src/infer/error_reporting/note.rs:247:33
  29:        0x113f6a81c - <rustc_infer[354a95a148735087]::infer::error_reporting::TypeErrCtxt>::report_region_errors
                               at /rustc/rust2/compiler/rustc_infer/src/infer/error_reporting/mod.rs:459:29
  30:        0x113d16fe0 - <rustc_trait_selection[292d5665640db603]::traits::engine::ObligationCtxt>::resolve_regions_and_report_errors
                               at /rustc/rust2/compiler/rustc_trait_selection/src/traits/engine.rs:196:17
  31:        0x1126b4890 - rustc_hir_analysis[20d141ca3328fe63]::check::compare_impl_item::check_type_bounds
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/check/compare_impl_item.rs:2172:5
  32:        0x11263bcb4 - rustc_hir_analysis[20d141ca3328fe63]::check::check::check_item_type
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/check/check.rs:717:75
  33:        0x1126416fc - rustc_hir_analysis[20d141ca3328fe63]::check::check::check_mod_item_types
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/check/check.rs:1559:9
  61:        0x1125af100 - rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}::{closure#0}
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/lib.rs:237:44
  62:        0x1125af100 - <rustc_middle[ac34d1cf85a491af]::hir::map::Map>::for_each_module::<rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}::{closure#0}>
                               at /rustc/rust2/compiler/rustc_middle/src/hir/map/mod.rs:622:13
  63:        0x1127283f0 - rustc_hir_analysis[20d141ca3328fe63]::check_crate::{closure#6}
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/lib.rs:237:9
  66:        0x112671a1c - rustc_hir_analysis[20d141ca3328fe63]::check_crate
                               at /rustc/rust2/compiler/rustc_hir_analysis/src/lib.rs:236:5
  67:        0x10ff51db0 - rustc_interface[8bdfa1e99f215169]::passes::analysis
                               at /rustc/rust2/compiler/rustc_interface/src/passes.rs:788:5
  95:        0x10fdce8d8 - <rustc_middle[ac34d1cf85a491af]::ty::context::TyCtxt>::analysis
                               at /rustc/rust2/compiler/rustc_middle/src/query/plumbing.rs:376:17

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.73.0-dev running on aarch64-apple-darwin

query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0261, E0601.
For more information about an error, try `rustc --explain E0261`.
```

</details>

<details>
<summary>Short backtrace (filtered with <code>rustc-backtrace-filter</code>)</summary>

```
error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:20
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                    ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0261]: use of undeclared lifetime name `'a`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:4:42
  |
4 |     fn bar<'other: 'a>() -> impl Sized + 'a {}
  |                                          ^^ undeclared lifetime
  |
help: consider introducing lifetime `'a` here
  |
4 |     fn bar<'a, 'other: 'a>() -> impl Sized + 'a {}
  |            +++
help: consider introducing lifetime `'a` here
  |
3 | trait Foo<'a> {
  |          ++++

error[E0601]: `main` function not found in crate `rpit_issue_113794`
 --> tests/ui/impl-trait/rpit-issue-113794.rs:5:2
  |
5 | }
  |  ^ consider adding a `main` function to `tests/ui/impl-trait/rpit-issue-113794.rs`

error: internal compiler error: compiler/rustc_middle/src/ty/mod.rs:2273:13: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: TypeNs("Foo"), disambiguator: 0 }, DisambiguatedDefPathData { data: ImplTraitAssocTy, disambiguator: 0 }], krate: crate0 }

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/rust2/compiler/rustc_errors/src/lib.rs:1645:9
stack backtrace:
   2: <rustc_errors::HandlerInner>::bug::<alloc::string::String>
             at ./compiler/rustc_errors/src/lib.rs:1645:9
   3: <rustc_errors::Handler>::bug::<alloc::string::String>
             at ./compiler/rustc_errors/src/lib.rs:1058:9
   4: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>::{closure#0}
             at ./compiler/rustc_middle/src/util/bug.rs:35:34
   8: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
             at ./compiler/rustc_middle/src/util/bug.rs:31:5
   9: rustc_middle::util::bug::bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:15:5
  10: <rustc_middle::ty::context::TyCtxt>::item_name::{closure#0}
             at ./compiler/rustc_middle/src/ty/mod.rs:2273:13
  11: <core::option::Option<rustc_span::symbol::Symbol>>::unwrap_or_else::<<rustc_middle::ty::context::TyCtxt>::item_name::{closure#0}>
             at ./library/core/src/option.rs:979:21
  12: <rustc_middle::ty::context::TyCtxt>::item_name
             at ./compiler/rustc_middle/src/ty/mod.rs:2272:9
  13: <rustc_infer::infer::error_reporting::TypeErrCtxt>::report_concrete_failure
             at ./compiler/rustc_infer/src/infer/error_reporting/note.rs:247:33
  14: <rustc_infer::infer::error_reporting::TypeErrCtxt>::report_region_errors
             at ./compiler/rustc_infer/src/infer/error_reporting/mod.rs:459:29
  15: <rustc_trait_selection::traits::engine::ObligationCtxt>::resolve_regions_and_report_errors
             at ./compiler/rustc_trait_selection/src/traits/engine.rs:196:17
  16: rustc_hir_analysis::check::compare_impl_item::check_type_bounds
             at ./compiler/rustc_hir_analysis/src/check/compare_impl_item.rs:2172:5
  17: rustc_hir_analysis::check::check::check_item_type
             at ./compiler/rustc_hir_analysis/src/check/check.rs:717:75
  18: rustc_hir_analysis::check::check::check_mod_item_types
             at ./compiler/rustc_hir_analysis/src/check/check.rs:1559:9
  19: rustc_query_impl::query_impl::check_mod_item_types::dynamic_query::{closure#2}::{closure#0}
             at ./compiler/rustc_query_impl/src/plumbing.rs:582:47
  20: rustc_middle::query::plumbing::query_ensure::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 0]>>>
             at ./compiler/rustc_middle/src/query/plumbing.rs:171:9
  21: <rustc_middle::query::plumbing::TyCtxtEnsure>::check_mod_item_types::<rustc_span::def_id::LocalDefId>
             at ./compiler/rustc_middle/src/query/plumbing.rs:346:17
  22: rustc_hir_analysis::check_crate::{closure#6}::{closure#0}
             at ./compiler/rustc_hir_analysis/src/lib.rs:237:44
  23: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#6}::{closure#0}>
             at ./compiler/rustc_middle/src/hir/map/mod.rs:622:13
  24: rustc_hir_analysis::check_crate::{closure#6}
             at ./compiler/rustc_hir_analysis/src/lib.rs:237:9
  27: rustc_hir_analysis::check_crate
             at ./compiler/rustc_hir_analysis/src/lib.rs:236:5
  28: rustc_interface::passes::analysis
             at ./compiler/rustc_interface/src/passes.rs:788:5
  29: rustc_query_impl::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}
             at ./compiler/rustc_query_impl/src/plumbing.rs:582:47
  30: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
             at ./compiler/rustc_middle/src/query/plumbing.rs:155:17
  31: <rustc_middle::query::plumbing::TyCtxtAt>::analysis
             at ./compiler/rustc_middle/src/query/plumbing.rs:385:31
  32: <rustc_middle::ty::context::TyCtxt>::analysis
             at ./compiler/rustc_middle/src/query/plumbing.rs:376:17

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.73.0-dev running on aarch64-apple-darwin

query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0261, E0601.
For more information about an error, try `rustc --explain E0261`.
```

</details>

<br>

#### License
<sup>
Licensed under either of <a href="https://www.apache.org/licenses/LICENSE-2.0">Apache License,
Version 2.0</a> or <a href="https://opensource.org/licenses/MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
</sub>

<br>

#### Code of conduct
<sup>
When contributing or interacting with this project, we ask abide the
<a href="https://www.rust-lang.org/en-US/conduct.html">Rust Code of Conduct</a> and ask that you do
too.
</sup>