// password_manager_backend/src/category_favorite/category.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub(crate) name: String,
}

impl Category {
    pub(crate) fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
    
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
