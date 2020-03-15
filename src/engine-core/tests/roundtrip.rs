// Copyright 2019 The Model Authors. All rights reserved.
// Use of this source code is governed by the Apache License,
// Version 2.0, that can be found in the LICENSE file.

use std::fs::File;
use std::io::BufReader;

use engine_core;

static TEST_MODELS: &[&str] = &[
    "test/test-models/samples/SIR/SIR.xmile",
    "test/test-models/samples/SIR/SIR_reciprocal-dt.xmile",
    "test/test-models/samples/bpowers-hares_and_lynxes_modules/model.xmile",
    "test/test-models/samples/teacup/teacup.xmile",
    "test/test-models/samples/teacup/teacup_w_diagram.xmile",
    "test/test-models/tests/trig/test_trig.xmile",
    "test/test-models/tests/lookups_inline/test_lookups_inline.xmile",
    "test/test-models/tests/comparisons/comparisons.xmile",
    "test/test-models/tests/sqrt/test_sqrt.xmile",
    "test/test-models/tests/abs/test_abs.xmile",
    "test/test-models/tests/constant_expressions/test_constant_expressions.xmile",
    "test/test-models/tests/lookups/test_lookups.xmile",
    "test/test-models/tests/lookups/test_lookups_no-indirect.xmile",
    "test/test-models/tests/line_breaks/test_line_breaks.xmile",
    "test/test-models/tests/parentheses/test_parens.xmile",
    "test/test-models/tests/builtin_max/builtin_max.xmile",
    "test/test-models/tests/number_handling/test_number_handling.xmile",
    "test/test-models/tests/if_stmt/if_stmt.xmile",
    "test/test-models/tests/game/test_game.xmile",
    "test/test-models/tests/eval_order/eval_order.xmile",
    "test/test-models/tests/xidz_zidz/xidz_zidz.xmile",
    "test/test-models/tests/exponentiation/exponentiation.xmile",
    "test/test-models/tests/logicals/test_logicals.xmile",
    "test/test-models/tests/limits/test_limits.xmile",
    "test/test-models/tests/line_continuation/test_line_continuation.xmile",
    "test/test-models/tests/ln/test_ln.xmile",
    "test/test-models/tests/model_doc/model_doc.xmile",
    "test/test-models/tests/reference_capitalization/test_reference_capitalization.xmile",
    "test/test-models/tests/log/test_log.xmile",
    "test/test-models/tests/function_capitalization/test_function_capitalization.xmile",
    "test/test-models/tests/chained_initialization/test_chained_initialization.xmile",
    "test/test-models/tests/exp/test_exp.xmile",
    "test/test-models/tests/builtin_min/builtin_min.xmile",
];

#[test]
fn roundtrips_model() {
    use std::io::Write;

    for &path in TEST_MODELS {
        let file_path = format!("../../{}", path);
        writeln!(::std::io::stderr(), "model: {}", path).unwrap();

        let f = File::open(file_path).unwrap();
        let mut f = BufReader::new(f);

        let project = engine_core::Project::from_xmile_reader(&mut f);
        if let Err(ref err) = project {
            writeln!(::std::io::stderr(), "model '{}' error: {}", path, err).unwrap();
        }
        assert!(project.is_ok());
    }
}
