#![allow(clippy::trivially_copy_pass_by_ref)]

use serde::Serializer;
use thousands::Separable;

pub(crate) fn serialize_f64<S>(val: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let truncated_val = (*val * 100.0).round() / 100.0;

    serializer.serialize_str(truncated_val.separate_with_commas().as_str())
}

pub(crate) fn serialize_usize<S>(val: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(val.separate_with_commas().as_str())
}
