//! # select wrapper
//! ```code
//! SELECT [ VALUE ] @fields [ AS @alias ]
//! 	FROM @targets
//! 	[ WHERE @conditions ]
//! 	[ SPLIT [ AT ] @field ... ]
//! 	[ GROUP [ BY ] @fields ... ]
//! 	[ ORDER [ BY ]
//! 		@fields [
//! 			RAND()
//! 			| COLLATE
//! 			| NUMERIC
//! 		] [ ASC | DESC ] ...
//! 	] ]
//! 	[ LIMIT [ BY ] @limit ]
//! 	[ START [ AT ] @start ]
//! 	[ FETCH @fields ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use crate::{Condition, Table, TimeOut, Order};

// pub struct SelectWrapper<'w> {
//     field: Field,
//     table: Table,
//     condition: Option<Condition>,
//     split: Vec<&'w str>,
//     group: Vec<&'w str>,
//     order: Order<'w>,
//     limit: Option<usize>,
//     start: Option<usize>,
//     fetch: Vec<&'w str>,
//     timeout: Option<TimeOut>,
//     parallel: bool,
// }