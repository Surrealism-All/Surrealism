//! # UpdateWrapper
//!
//! ## example
//! ```code
//! UPDATE @targets
//! 	[ CONTENT @value
//! 	  | MERGE @value
//! 	  | PATCH @value
//! 	  | SET @field = @value ...
//! 	]
//! 	[ WHERE @condition ]
//! 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```

use crate::{Condition, ReturnType, TimeOut};
use crate::{ContentSet, Patch};
use super::{BaseWrapperImpl, ReturnImpl, ParallelImpl, TimeoutImpl};

pub struct UpdateWrapper<'w> {
    strategy: UpdateStrategy<'w>,
    condition: Condition,
    return_type: ReturnType,
    timeout: TimeOut,
    parallel: bool,
}

pub enum UpdateStrategy<'u> {
    Basic(ContentSet<'u>),
    Patch(Patch<'u>),
}