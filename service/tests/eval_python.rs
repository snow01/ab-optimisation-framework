// #[cfg(test)]
// mod tests {
//     use std::time::Instant;
//
//     // use anyhow::anyhow;
//     // use anyhow::Context;
//     use pyo3::prelude::*;
//     use pyo3::types::IntoPyDict;
//     use pythonize::pythonize;
//     use serde::{Deserialize, Serialize};
//     use serde_json::json;
//     use std::ops::Sub;
//
//     // use pyo3::types::IntoPyDict;
//
//     #[derive(Clone, Debug, Serialize, Deserialize)]
//     pub struct TrackedExperiment {
//         // pub short_name: String,
//         // pub selected_version: i64,
//         // pub selected_member_kind: ExperimentMemberKind,
//         // pub selected_variation: Option<String>,
//         pub selection_date: chrono::DateTime<chrono::Local>,
//         pub total_selection_count: u64,
//         // pub invocation_version: i64,
//         pub invocation_date: chrono::DateTime<chrono::Local>,
//         pub total_invocation_count: u64,
//     }
//
//     #[test]
//     fn test_pyo3() -> anyhow::Result<()> {
//         // for _i in 0..10 {
//         let instance = Instant::now();
//
//         Python::with_gil(|py| -> anyhow::Result<bool> {
//             let ctx = Some(json!({
//                 "new_user": true,
//                 "app_version": "4.8.3",
//             }));
//             let ctx = pythonize(py, &ctx)?;
//
//             let tracked_experiment = TrackedExperiment {
//                 // selected_version: 3,
//                 selection_date: chrono::Local::now().sub(chrono::Duration::days(15)),
//                 // invocation_version: 0
//                 total_selection_count: 3,
//                 invocation_date: chrono::Local::now().sub(chrono::Duration::days(15)),
//                 total_invocation_count: 4,
//             };
//
//             let tracked_experiment = pythonize(py, &tracked_experiment)?;
//
//             let locals = [("ctx", ctx), ("experiment", tracked_experiment)].into_py_dict(py);
//
//             // let sys = PyModule::import(py, "sys")?;
//             let fns = PyModule::from_code(
//                 py,
//                 r#"
// from packaging.version import parse as parse_version
// from datetime import date,time,datetime,timedelta
//
// def app_version(ctx):
//     return parse_version(ctx.get('app_version', '0.0.0'))
//
// def lt_version(ctx, version):
//     return app_version(ctx) < parse_version(version)
//
// def le_version(ctx, version):
//     return app_version(ctx) <= parse_version(version)
//
// def gt_version(ctx, version):
//     return app_version(ctx) > parse_version(version)
//
// def ge_version(ctx, version):
//     return app_version(ctx) >= parse_version(version)
//
// def eq_version(ctx, version):
//     return app_version(ctx) == parse_version(version)
//
// def ne_version(ctx, version):
//     return app_version(ctx) != parse_version(version)
//
// def allow_only_once(experiment):
//     return experiment is None or experiment.get('total_selection_count', 0) == 0
//
// def allow_max_x_times(experiment, times):
//     return experiment is None or experiment.get('total_selection_count', 0) < times
//
// def allow_every_x_times(experiment, times):
//     return experiment is None or experiment.get('total_invocation_count', 0) % times == 0
//
// def invocation_date(experiment):
//     return datetime.fromisoformat(experiment.get('invocation_date'))
//
// def selection_date(experiment):
//     return datetime.fromisoformat(experiment.get('selection_date'))
//
// def allow_once_per_x_period(experiment, weeks=0, days=0, hours=0, minutes=0, seconds=0):
//     if experiment is None or 'selection_date' not in experiment:
//         return false
//
//     selection_time = selection_date(experiment)
//     current_time = datetime.now(tz=selection_time.tzinfo)
//     diff = current_time - selection_time
//
//     return diff >= timedelta(weeks=weeks,days=days,hours=hours,minutes=minutes,seconds=seconds)
//                 "#,
//                 "fns.py",
//                 "fns",
//             )?;
//             let globals = [("fns", fns)].into_py_dict(py);
//
//             let result = py.eval(
//                 "fns.lt_version(ctx, '4.5.3')",
//                 // "fns.datetime.fromisoformat(experiment.get('selection_date'))",
//                 Some(&globals),
//                 Some(&locals),
//             )?;
//
//             println!("Result: {:#?}", result);
//
//             let result = py.eval(
//                 // "fns.datetime.fromisoformat(experiment.get('selection_date'))",
//                 "fns.allow_every_x_times(experiment, 2)",
//                 Some(globals),
//                 Some(locals),
//             );
//
//             println!("Result: {:#?}", result);
//
//             // println!("Total time taken: {:?}", instance.elapsed());
//
//             Ok(true)
//         })?;
//         // }
//
//         Ok(())
//     }
// }
