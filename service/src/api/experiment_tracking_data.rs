use std::fmt::{Display, Formatter};

use anyhow::{bail, Context};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use pest::Parser;
use serde::{Deserialize, Serialize};

use crate::api::common::ExperimentMemberKind;

#[derive(Parser)]
#[grammar = "api/tracking_cookie_parser.pest"]
pub struct ExperimentTrackingCookieParser;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackingData {
    pub experiments: Vec<Experiment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experiment {
    pub short_name: String,
    pub version: i64,
    pub member_kind: ExperimentMemberKind,
    pub variation: Option<String>,
}

impl Display for TrackingData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for experiment in self.experiments.iter() {
            if !first {
                write!(f, "~")?;
            }

            write!(f, "{}", experiment)?;
            first = false;
        }

        Ok(())
    }
}

impl Display for Experiment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.short_name,
            self.version,
            self.member_kind,
            self.variation.as_ref().map_or_else(|| "", |v| v)
        )
    }
}

impl ExperimentTrackingCookieParser {
    pub fn parse_str_no_err(value: &str) -> TrackingData {
        match ExperimentTrackingCookieParser::parse_str(value) {
            Ok(result) => result,
            Err(err) => {
                warn!("Error in parsing cookie: {} ==> {:?}", value, err);
                TrackingData {
                    experiments: vec![],
                }
            }
        }
    }

    pub fn parse_str(value: &str) -> anyhow::Result<TrackingData> {
        if value.is_empty() {
            return Ok(TrackingData {
                experiments: vec![],
            });
        }

        let cookie_rule = ExperimentTrackingCookieParser::parse(Rule::cookie, value)
            .with_context(|| format!("Error in parsing '{}' as ContextData", value))?
            .next()
            .unwrap();

        let mut experiments = Vec::<Experiment>::new();
        for experiment_rule in cookie_rule.into_inner() {
            if let Rule::experiment = experiment_rule.as_rule() {
                let mut short_name = "";
                let mut version: i64 = 0;
                let mut member_kind = ExperimentMemberKind::Control;
                let mut variation = None;

                for inner in experiment_rule.into_inner() {
                    match inner.as_rule() {
                        Rule::short_name => {
                            short_name = inner.as_str();
                        }
                        Rule::version => {
                            let version_str = inner.as_str();
                            version = version_str.parse::<i64>()?;
                        }
                        Rule::member_kind => {
                            let member_kind_str = inner.as_str();
                            match member_kind_str {
                                "C" => member_kind = ExperimentMemberKind::Control,
                                "T" => member_kind = ExperimentMemberKind::Test,

                                #[allow(unreachable_code)]
                                _ => {
                                    return Err(bail!("Unknown member kind found"));
                                }
                            }
                        }
                        Rule::variation => {
                            variation = Some(inner.as_str().to_string());
                        }

                        #[allow(unreachable_code)]
                        _ => {
                            return Err(bail!("Unknown rule found"));
                        }
                    }
                }

                let experiment = Experiment {
                    short_name: short_name.to_string(),
                    version,
                    member_kind,
                    variation,
                };

                experiments.push(experiment);
            }
        }

        let cookie_data = TrackingData { experiments };

        return Ok(cookie_data);
    }
}

#[cfg(test)]
mod tests {
    use crate::api::experiment_tracking_data::ExperimentTrackingCookieParser;

    #[test]
    fn parse() -> anyhow::Result<()> {
        ExperimentTrackingCookieParser::parse_str("a|0|T|;a|0|C|d")?;
        Ok(())
    }
}
