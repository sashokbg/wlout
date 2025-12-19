use crate::common::HeadModeInput;
use clap::builder::TypedValueParser;
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Command, Error};
use std::ffi::OsStr;

#[derive(Debug, Clone, Copy)]
pub struct DisplayModeParser {}

impl TypedValueParser for DisplayModeParser {
    type Value = HeadModeInput;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        let parts = value.to_str().unwrap().split("@").collect::<Vec<&str>>();
        if parts.len() != 2 {
            let mut error = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
            error.insert(
                ContextKind::InvalidArg,
                ContextValue::String(arg.unwrap().to_string()),
            );

            return Err(error);
        }

        let size_part = parts[0];
        let rate_part = parts[1];

        let size_split = size_part.split('x').collect::<Vec<&str>>();

        if size_split.len() != 2 {
            let mut error = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
            error.insert(
                ContextKind::InvalidArg,
                ContextValue::String(arg.unwrap().to_string()),
            );

            return Err(error);
        }

        Ok(HeadModeInput {
            rate: rate_part
                .parse::<i32>()
                .expect("Rate should be a valid integer"),
            width: size_split[0]
                .parse::<i32>()
                .expect("Width should be a valid integer"),
            height: size_split[1]
                .parse::<i32>()
                .expect("Height should be a valid integer"),
        })
    }
}
