use std::fmt;
use std::fmt::{Display, Formatter};

use anyhow::{bail, Context};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use pest::Parser;
use serde::de::{self as serde_de, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::common::ExperimentMemberKind;

static FIRST_JULY_2021: i64 = 1625097600;

#[derive(Parser)]
#[grammar = "api/tracking_cookie_parser.pest"]
pub struct TrackingDataParser;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackingData {
    pub experiments: Vec<TrackedExperiment>,
}

#[derive(Clone, Debug)]
pub struct TrackedExperiment {
    pub short_name: String,
    pub selected_version: i64,
    pub selected_member_kind: ExperimentMemberKind,
    pub selected_variation: Option<String>,
    pub selection_date: i64,
    pub total_selection_count: u64,
    pub invocation_version: i64,
    pub invocation_date: i64,
    pub total_invocation_count: u64,
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

impl Display for TrackedExperiment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.short_name,
            self.selected_version,
            self.selected_member_kind,
            self.selected_variation.as_ref().map_or_else(|| "", |v| v),
            self.selection_date - FIRST_JULY_2021,
            self.total_selection_count,
            self.invocation_version,
            self.invocation_date - self.selection_date,
            self.total_invocation_count
        )
    }
}

impl Serialize for TrackedExperiment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TrackedExperiment {
    fn deserialize<D>(deserializer: D) -> Result<TrackedExperiment, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = TrackedExperiment;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde_de::Error,
            {
                TrackingDataParser::parse_experiment(&value).map_err(|e| {
                    serde_de::Error::custom(format!(
                        "Error in deserializing string {} ==> {:?}",
                        value, e
                    ))
                })
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl TrackingDataParser {
    pub fn parse_tracking_data_no_err(value: &str) -> TrackingData {
        match TrackingDataParser::parse_tracking_data(value) {
            Ok(result) => result,
            Err(err) => {
                warn!("Error in parsing cookie: {} ==> {:?}", value, err);
                TrackingData {
                    experiments: vec![],
                }
            }
        }
    }

    pub fn parse_tracking_data(value: &str) -> anyhow::Result<TrackingData> {
        if value.is_empty() {
            return Ok(TrackingData {
                experiments: vec![],
            });
        }

        let cookie_rule = TrackingDataParser::parse(Rule::cookie, value)
            .with_context(|| format!("Error in parsing '{}' as TrackingData", value))?
            .next()
            .unwrap();

        let mut experiments = Vec::<TrackedExperiment>::new();
        for experiment_rule in cookie_rule.into_inner() {
            if let Rule::experiment = experiment_rule.as_rule() {
                experiments.push(Self::parse_experiment_rule(experiment_rule)?);
            }
        }

        let cookie_data = TrackingData { experiments };

        return Ok(cookie_data);
    }

    pub fn parse_experiment(value: &str) -> anyhow::Result<TrackedExperiment> {
        // if value.is_empty() {
        //     return bail!("Experiment string is empty");
        // }

        let experiment_rule = TrackingDataParser::parse(Rule::experiment, value)
            .with_context(|| format!("Error in parsing '{}' as ContextData", value))?
            .next()
            .unwrap();

        Self::parse_experiment_rule(experiment_rule)
    }

    fn parse_experiment_rule(
        experiment_rule: pest::iterators::Pair<Rule>,
    ) -> anyhow::Result<TrackedExperiment> {
        let mut short_name = "";
        let mut selected_version: i64 = 0;
        let mut selected_member_kind = ExperimentMemberKind::Control;
        let mut selected_variation = None;
        let mut selection_date = 0;
        let mut total_selection_count = 0;
        let mut invocation_version = 0;
        let mut invocation_date = 0;
        let mut total_invocation_count = 0;

        for inner in experiment_rule.into_inner() {
            match inner.as_rule() {
                Rule::short_name => {
                    short_name = inner.as_str();
                }

                Rule::selected_version => {
                    let version_str = inner.as_str();
                    selected_version = version_str.parse::<i64>()?;
                }

                Rule::selected_member_kind => {
                    let member_kind_str = inner.as_str();
                    match member_kind_str {
                        "C" => selected_member_kind = ExperimentMemberKind::Control,
                        "T" => selected_member_kind = ExperimentMemberKind::Test,

                        #[allow(unreachable_code)]
                        _ => {
                            return Err(bail!("Unknown member kind found"));
                        }
                    }
                }

                Rule::selected_variation => {
                    selected_variation = Some(inner.as_str().to_string());
                }

                Rule::selection_date => {
                    // add first create_date
                    selection_date = inner.as_str().parse::<i64>()? + FIRST_JULY_2021;
                }

                Rule::total_selection_count => {
                    total_selection_count = inner.as_str().parse::<u64>()?;
                }

                Rule::invocation_version => {
                    let version_str = inner.as_str();
                    invocation_version = version_str.parse::<i64>()?;
                }

                Rule::invocation_date => {
                    invocation_date = inner.as_str().parse::<i64>()? + selection_date;
                }

                Rule::total_invocation_count => {
                    total_invocation_count = inner.as_str().parse::<u64>()?;
                }

                #[allow(unreachable_code)]
                _ => {
                    return Err(bail!("Unknown rule found"));
                }
            }
        }

        Ok(TrackedExperiment {
            short_name: short_name.to_string(),
            selected_version,
            selected_member_kind,
            selected_variation,
            selection_date,
            total_selection_count,
            invocation_version,
            invocation_date,
            total_invocation_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::api::experiment_tracking_data::TrackingDataParser;

    #[test]
    fn parse() -> anyhow::Result<()> {
        TrackingDataParser::parse_tracking_data("a|0|T|;a|0|C|d")?;
        Ok(())
    }
}
