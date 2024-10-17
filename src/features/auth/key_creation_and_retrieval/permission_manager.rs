use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::core::errors::auth_errors::AuthError;
use crate::features::auth::key_creation_and_retrieval::claims;



#[derive(Debug)]
pub struct PermissionsManager {
    pub admin_permissions: FeaturePermissions,
    pub user_permissions: FeaturePermissions,
    pub equipment_permissions: FeaturePermissions,
}

#[async_trait]
impl<S> FromRequestParts<S> for PermissionsManager
where S: Send + Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims_result = claims::Claims::from_request_parts(parts, _state).await;
        let claims = match claims_result {
            Ok(claims) => claims,
            Err(_) => return Err(AuthError::InvalidToken),
        };
        // extract permissions from claims permissions_bitwise
        let permissions = PermissionsManager::from_permissions_bitwise(&claims.permissions_bitwise);
        Ok(permissions)
    }
}

pub const PERMISSION_WIDTH: usize = 4;
impl PermissionsManager {
    pub fn new() -> Self {
        PermissionsManager {
            admin_permissions: FeaturePermissions::new(),
            user_permissions: FeaturePermissions::new(),
            equipment_permissions: FeaturePermissions::new(),
        }
    }
    pub fn from_permissions_bitwise(permissions_bitwise: &String) -> Self {
        let permissions: Vec<_> = permissions_bitwise.split("-").collect();
        
        Some(PermissionsManager {
            admin_permissions: FeaturePermissions::from_string(&permissions[0].to_string()).unwrap(),
            user_permissions: FeaturePermissions::from_string(&permissions[1].to_string()).unwrap(),
            equipment_permissions: FeaturePermissions::from_string(&permissions[2].to_string()).unwrap(),
        }).unwrap_or_else(|| PermissionsManager::new())
    }
    pub fn to_permissions_bitwise(&self) -> String {
        let mut permissions_bitwise = String::new();

        permissions_bitwise.push_str(format!("{:0width$b}", self.admin_permissions.bits, width = PERMISSION_WIDTH).as_str());
        permissions_bitwise.push_str("-");
        permissions_bitwise.push_str(format!("{:0width$b}", self.user_permissions.bits, width = PERMISSION_WIDTH).as_str());
        permissions_bitwise.push_str("-");
        permissions_bitwise.push_str(format!("{:0width$b}", self.equipment_permissions.bits, width = PERMISSION_WIDTH).as_str());

        permissions_bitwise

    }
}

#[derive(Debug, Clone, Copy)]
pub struct FeaturePermissions {
    pub bits: u32,
}

impl FeaturePermissions {
    fn new() -> Self {
        FeaturePermissions { bits: 0 }
    }
    
    fn from_string(permissions_bitwise: &String) -> Result<Self, &'static str> {
        if let Ok(bits) = u32::from_str_radix(permissions_bitwise, 2) {
            Ok(FeaturePermissions { bits })
        } else {
            Err("Invalid permission string format. Only binary strings allowed.")
        }
    }


    pub fn set_permission(&mut self, permission: u32) {
        self.bits |= permission;
    }

    pub fn remove_permission(&mut self, permission: u32) {
        self.bits &= !permission;
    }

    pub fn has_permission(&self, permission: u32) -> bool {
        (self.bits & permission) != 0
    }

    pub fn set_permission_group(&mut self, group: u32) {
        self.bits |= group;
    }

    pub fn remove_permission_group(&mut self, group: u32) {
        self.bits &= !group;
    }

    pub fn has_permission_group(&self, group: u32) -> bool {
        (self.bits & group) == group
    }

    pub fn clear_permissions(&mut self) {
        self.bits = 0;
    }
}