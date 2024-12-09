// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum OrphanedV1Enum {
    First,
Second,
Third,
}

impl ::std::fmt::Display for OrphanedV1Enum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            OrphanedV1Enum::First => write!(f, "FIRST"),
OrphanedV1Enum::Second => write!(f, "SECOND"),
OrphanedV1Enum::Third => write!(f, "THIRD"),
        }
    }
}
