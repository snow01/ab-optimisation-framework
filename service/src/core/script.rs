use std::fmt;

use anyhow::Context;
use serde::de::{self as serde_de, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

pub struct Script {
    pub src: String,
    pub expression: jexl_parser::ast::Expression,
}

impl PartialEq<Self> for Script {
    fn eq(&self, other: &Self) -> bool {
        self.src.eq(&other.src)
    }
}

impl Eq for Script {}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

impl fmt::Debug for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

pub struct ScriptEvaluator {
    pub evaluator: jexl_eval::Evaluator,
}

impl ScriptEvaluator {
    pub fn new() -> Self {
        // todo: do we add functions here

        let evaluator = jexl_eval::Evaluator::new();
        ScriptEvaluator { evaluator }
    }

    pub fn evaluate(&self, script: Option<&Script>, ctx: &jexl_eval::Value) -> anyhow::Result<bool> {
        match script {
            None => Ok(true),
            Some(script) => {
                let value = self
                    .evaluator
                    .eval_ast(script.expression.clone(), ctx)
                    .with_context(|| format!("Error in evaluating script={} with context={:?}", script.src, ctx))?;
                if let jexl_eval::Value::Bool(value) = value {
                    Ok(value)
                } else {
                    Err(anyhow::anyhow!(
                        "Got not a boolean result in evaluating script={} with context={:?}",
                        script.src,
                        ctx
                    ))
                }
            }
        }
    }
}

impl Serialize for Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.src.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Script;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("valid script expression")
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Script, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, src: String) -> Result<Script, E>
            where
                E: serde_de::Error,
            {
                jexl_parser::Parser::parse(&src)
                    .map_err(|e| serde_de::Error::custom(format!("Error in deserializing string {} to jexl expression ==> {:?}", src, e)))
                    .and_then(|expression| Ok(Script { src, expression }))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
