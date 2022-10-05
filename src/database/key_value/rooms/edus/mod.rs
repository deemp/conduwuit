mod presence;
mod typing;
mod read_receipt;

use std::sync::Arc;

use crate::{service, database::KeyValueDatabase};

impl service::rooms::edus::Data for Arc<KeyValueDatabase> {}
