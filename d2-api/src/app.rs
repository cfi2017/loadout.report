use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

#[bitflags]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationScopes {
    /// Read basic user profile information such as the user's handle, avatar icon, etc.
    ReadBasicUserProfile = 1,
    /// Read Group/Clan Forums, Wall, and Members for groups and clans that the user has joined.
    ReadGroups = 2,
    /// Write Group/Clan Forums, Wall, and Members for groups
    /// and clans that the user has joined.
    WriteGroups = 4,
    /// Administer Group/Clan Forums, Wall, and
    /// Members for groups and clans that the user has joined.
    AdminGroups = 8,
    /// Create new groups, clans and forum posts, along with other actions that are reserved for
    /// Bungie.net elevated scope: not meant to be used by third party applications.
    BnetWrite = 16,
    /// Move or equip Destiny 2 items.
    MoveEquipDestinyItems = 32,
    /// Read Destiny 1 Inventory and Vault contents. For Destiny 2, this scope is needed to read
    /// anything regarded as private. This is the only scope a Destiny 2 app needs for read
    /// operations against Destiny 2 data such as inventory, vault, currency, vendors,
    /// milestones, progression, etc.
    ReadDestinyInventoryAndVault = 64,
    /// Read user data such as who they are, web notifications, clan/group memberships,
    /// recent activity, muted users.
    ReadUserData = 128,
    /// Edit user data such as preferred language, status, motto, avatar selection and theme.
    EditUserData = 256,
    /// Access Vendor and advisor data specific to a user. OBSOLETE. This scope is only used on
    /// the Destiny 1 API.
    #[deprecated(note = "deprecated in accordance with API spec")]
    ReadDestinyVendorsAndAdvisors = 512,
    /// Read offer history and claim and apply tokens for the user.
    ReadAndApplyTokens = 1024,
    /// Can perform actions that will result in a prompt to the user via the Destiny app.
    AdvancedWriteActions = 2048,
    /// Can use the partner offer api to claim rewards defined for a partner.
    PartnerOfferGrant = 4096,
    /// Allows an app to query sensitive information like unlock flags and values not available
    /// through normal methods.
    DestinyUnlockValueQuery = 8192,
    /// Allows an app to query sensitive user PII, most notably email information.
    UserPiiRead = 16384,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiUsage {
    /// Counts for on API calls made for the time range.
    pub api_calls: Vec<Series>,
    /// Instances of blocked requests
    /// or requests that crossed the warn threshold during the time range.
    pub throttled_requests: Vec<Series>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Series {
    /// Collection of samples with time and value.
    pub datapoints: Vec<Datapoint>,
    /// Target to which datapoints apply.
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Datapoint {
    /// Timestamp for the related count.
    pub time: chrono::DateTime<chrono::Utc>,
    /// Count associated with timestamp.
    /// Side note, why is this nullable?
    pub count: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Application {
    /// Unique ID assigned to the application.
    #[serde(rename = "applicationId")]
    pub id: i32,
    /// Name of the application.
    pub name: String,
    /// URL used to pass the user's authorization code to the application.
    pub redirect_url: String,
    /// Link to website for the application where a user can learn more about the app.
    pub link: String,
    /// Permissions the application needs to work.
    pub scope: BitFlags<ApplicationScopes>,
    /// Value of the Origin header sent in requests generated by this application.
    pub origin: String,
    /// Current status of the application.
    pub status: ApplicationStatus,
    /// Date the application was first added to our database.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// Date the application status last changed.
    pub status_changed: chrono::DateTime<chrono::Utc>,
    /// Date the first time the application entered the 'Public' status.
    pub first_published: chrono::DateTime<chrono::Utc>,
    /// List of team members who manage this application on Bungie.n et.
    /// Will always consist of at least the application owner.
    pub team: Vec<ApplicationDeveloper>,
    /// An optional override for the Authorize view name.
    pub override_authorize_view_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    /// No value assigned.
    None = 0,
    /// Application exists and works but will not appear in any public catalog. New applications
    /// start in this state, test applications will remain in this state.
    Private = 1,
    /// Active applications that can appear in a catalog.
    Public = 2,
    /// Applications disabled by the owner. All authorizations will be treated as terminated
    /// while in this state. Owner can move back to private or public state.
    Disabled = 3,
    /// Application has been blocked by Bungie.
    /// It cannot be transitioned out of this state by the owner.
    /// Authorizations are terminated when an application is in this state.
    Blocked = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApplicationDeveloper {
    pub role: DeveloperRole,
    pub api_eula_version: i32,
    pub user: crate::user::InfoCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeveloperRole {
    None = 0,
    Owner = 1,
    TeamMember = 2,
}
