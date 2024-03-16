use serde::Deserialize;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Deserialize)]
pub enum AdminPermissions {
    CanSignIn = 0,
    CanRecoverAccount = 1,
    CanManageUsers = 2,
    CanManageRoles = 3,
    CanManageLanguages = 4,
    CanManageCharacterSet = 5,
    CanManagePhonetics = 6,
    CanManageLessonTemplates = 7,
    CanManageLessons = 8,
    CanManageVocabulary = 9,
    CanManageQuizzes = 10,
    CanViewFeedback = 11,
    CanViewReports = 12,
    CanProvideSupport = 13,
}

impl From<i32> for AdminPermissions {
    fn from(permission: i32) -> Self {
        match permission {
            0 => AdminPermissions::CanSignIn,
            1 => AdminPermissions::CanRecoverAccount,
            2 => AdminPermissions::CanManageUsers,
            3 => AdminPermissions::CanManageRoles,
            4 => AdminPermissions::CanManageLanguages,
            5 => AdminPermissions::CanManageCharacterSet,
            6 => AdminPermissions::CanManagePhonetics,
            7 => AdminPermissions::CanManageLessonTemplates,
            8 => AdminPermissions::CanManageLessons,
            9 => AdminPermissions::CanManageVocabulary,
            10 => AdminPermissions::CanManageQuizzes,
            11 => AdminPermissions::CanViewFeedback,
            12 => AdminPermissions::CanViewReports,
            13 => AdminPermissions::CanProvideSupport,
            _ => AdminPermissions::CanSignIn,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct AdminPermission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission: AdminPermissions,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct CreateAdminPermission {
    pub user_id: Uuid,
    pub permission: AdminPermissions,
}

#[derive(Clone)]
pub struct UpdateAdminPermission {
    pub user_id: Uuid,
    pub permission: AdminPermissions,
}
