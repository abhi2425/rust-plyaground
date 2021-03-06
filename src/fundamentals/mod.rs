mod arrays;
mod associated_types;
mod closures;
mod conversions;
mod expressions;
mod flow_control;
mod flow_match;
mod format;
mod generics;
mod iterators;
mod lifetimes;
mod literals_operators;
mod modules;
mod scoping_rules;
mod structs;
mod traits;
mod tuples;
mod types;
mod vars;

pub use arrays::run as run_arrays;
pub use associated_types::run as run_associated_types;
pub use closures::run as run_closures;
pub use conversions::run as run_conversion;
pub use expressions::run as run_expressions;
pub use flow_control::run as run_flow_control;
pub use flow_match::run as run_flow_match;
pub use format::run as run_format;
pub use generics::run as run_generics;
pub use iterators::run as run_iterators;
pub use lifetimes::run as run_lifetimes;
pub use literals_operators::run as run_literals_operators;
pub use modules::run as run_modules;
pub use scoping_rules::run as run_scoping_rules;
pub use structs::run as run_structs;
pub use traits::run as run_traits;
pub use tuples::run as run_tuples;
pub use types::run as run_types;
pub use vars::run as run_vars;
