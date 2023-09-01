use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rocket::serde::json::Value;

pub type UUID = String;
pub type RecordData = HashMap<String, Value>;

/** A JSON representation of a record with data, info, and acls */
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordJson {
    /** The data of the record */
    pub meta: RecordMeta,

    /** The data of the record */
    pub data: RecordData,

    /** The information about the record */
    pub info: RecordInfo,
    
    /** The access control lists of the record */
    pub acls: RecordAcls,
}

/** The data of a record with id, namespace, and security classification */
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordMeta {
    /** The ID of the record */
    pub id: Option<String>,

    /** The namespace of the record */
    pub ns: Option<String>,
    
    /** The security classification of the record */
    pub sc: Option<String>,
}

/** The information about a record including timestamps and user IDs for various actions */
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordInfo {
    /** The timestamp when the record was created */
    pub created_at: Option<String>,
    
    /** The ID of the user who created the record */
    pub created_by: Option<String>,
    
    /** The timestamp when the record was last updated */
    pub updated_at: Option<String>,
    
    /** The ID of the user who last updated the record */
    pub updated_by: Option<String>,
    
    /** The timestamp when the record expired */
    pub expired_at: Option<String>,
    
    /** The ID of the user who set the record to expire */
    pub expired_by: Option<String>,
    
    /** The timestamp when the record was deleted */
    pub deleted_at: Option<String>,
    
    /** The ID of the user who deleted the record */
    pub deleted_by: Option<String>,
}

/** The access control lists of a record including full, edit, read, and deny lists */
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordAcls {
    /** List of security IDs that have unrestricted access to this record */
    pub acls_full: Vec<UUID>,

    /** List of security IDs that can make record data changes */
    pub acls_edit: Vec<UUID>,

    /** List of security IDs that can explicitly read record data */
    pub acls_read: Vec<UUID>,

    /** List of security IDs that are explicitly blacklisted from even knowing the record exists */
    pub acls_deny: Vec<UUID>,
}