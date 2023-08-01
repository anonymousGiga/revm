use minitrace::prelude::*;
use tracing::info;

use minitrace::collector::Reporter;

pub(crate) struct MyReporter;

impl Reporter for MyReporter {
    fn report(&mut self, spans: &[SpanRecord]) {
        for v in spans {
            if v.name == "sload" {
                info!(
                    target : "revm-test-sload",
                    "v.name = {:?}, v.duration_ns = {:?}",
                    v.name,
                    v.duration_ns,
                );
            }
        }
    }
}
