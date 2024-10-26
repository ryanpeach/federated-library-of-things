#[cfg(feature = "full")]
use crate::schema::{community, community_follower, community_moderator, community_person_ban};
use crate::{
  newtypes::{CommunityId, DbUrl, InstanceId, PersonId},
  sensitive::SensitiveString,
  source::placeholder_apub_url,
  CommunityVisibility,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[cfg(feature = "full")]
use ts_rs::TS;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Selectable, Identifiable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = community))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", ts(export))]
/// A community.
pub struct Community {
  pub id: CommunityId,
  pub name: String,
  /// A longer title, that can contain other characters, and doesn't have to be unique.
  pub title: String,
  /// A sidebar / markdown description.
  pub description: Option<String>,
  /// Whether the community is removed by a mod.
  pub removed: bool,
  pub published: DateTime<Utc>,
  pub updated: Option<DateTime<Utc>>,
  /// Whether the community has been deleted by its creator.
  pub deleted: bool,
  /// Whether its an NSFW community.
  pub nsfw: bool,
  /// The federated actor_id.
  pub actor_id: DbUrl,
  /// Whether the community is local.
  pub local: bool,
  #[serde(skip)]
  pub private_key: Option<SensitiveString>,
  #[serde(skip)]
  pub public_key: String,
  #[serde(skip)]
  pub last_refreshed_at: DateTime<Utc>,
  /// A URL for an icon.
  pub icon: Option<DbUrl>,
  /// A URL for a banner.
  pub banner: Option<DbUrl>,
  #[cfg_attr(feature = "full", ts(skip))]
  #[serde(skip)]
  pub followers_url: Option<DbUrl>,
  #[cfg_attr(feature = "full", ts(skip))]
  #[serde(skip, default = "placeholder_apub_url")]
  pub inbox_url: DbUrl,
  /// Whether the community is hidden.
  pub hidden: bool,
  /// Whether posting is restricted to mods only.
  pub posting_restricted_to_mods: bool,
  pub instance_id: InstanceId,
  /// Url where moderators collection is served over Activitypub
  #[serde(skip)]
  pub moderators_url: Option<DbUrl>,
  /// Url where featured posts collection is served over Activitypub
  #[serde(skip)]
  pub featured_url: Option<DbUrl>,
  pub visibility: CommunityVisibility,
}

#[derive(Debug, Clone, derive_new::new)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = community))]
pub struct CommunityInsertForm {
  pub instance_id: InstanceId,
  pub name: String,
  pub title: String,
  pub public_key: String,
  #[new(default)]
  pub description: Option<String>,
  #[new(default)]
  pub removed: Option<bool>,
  #[new(default)]
  pub published: Option<DateTime<Utc>>,
  #[new(default)]
  pub updated: Option<DateTime<Utc>>,
  #[new(default)]
  pub deleted: Option<bool>,
  #[new(default)]
  pub nsfw: Option<bool>,
  #[new(default)]
  pub actor_id: Option<DbUrl>,
  #[new(default)]
  pub local: Option<bool>,
  #[new(default)]
  pub private_key: Option<String>,
  #[new(default)]
  pub last_refreshed_at: Option<DateTime<Utc>>,
  #[new(default)]
  pub icon: Option<DbUrl>,
  #[new(default)]
  pub banner: Option<DbUrl>,
  #[new(default)]
  pub followers_url: Option<DbUrl>,
  #[new(default)]
  pub inbox_url: Option<DbUrl>,
  #[new(default)]
  pub moderators_url: Option<DbUrl>,
  #[new(default)]
  pub featured_url: Option<DbUrl>,
  #[new(default)]
  pub hidden: Option<bool>,
  #[new(default)]
  pub posting_restricted_to_mods: Option<bool>,
  #[new(default)]
  pub visibility: Option<CommunityVisibility>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = community))]
pub struct CommunityUpdateForm {
  pub title: Option<String>,
  pub description: Option<Option<String>>,
  pub removed: Option<bool>,
  pub published: Option<DateTime<Utc>>,
  pub updated: Option<Option<DateTime<Utc>>>,
  pub deleted: Option<bool>,
  pub nsfw: Option<bool>,
  pub actor_id: Option<DbUrl>,
  pub local: Option<bool>,
  pub public_key: Option<String>,
  pub private_key: Option<Option<String>>,
  pub last_refreshed_at: Option<DateTime<Utc>>,
  pub icon: Option<Option<DbUrl>>,
  pub banner: Option<Option<DbUrl>>,
  pub followers_url: Option<DbUrl>,
  pub inbox_url: Option<DbUrl>,
  pub moderators_url: Option<DbUrl>,
  pub featured_url: Option<DbUrl>,
  pub hidden: Option<bool>,
  pub posting_restricted_to_mods: Option<bool>,
  pub visibility: Option<CommunityVisibility>,
}

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(
  feature = "full",
  derive(Identifiable, Queryable, Selectable, Associations)
)]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::community::Community))
)]
#[cfg_attr(feature = "full", diesel(table_name = community_moderator))]
#[cfg_attr(feature = "full", diesel(primary_key(person_id, community_id)))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct CommunityModerator {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub published: DateTime<Utc>,
}

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = community_moderator))]
pub struct CommunityModeratorForm {
  pub community_id: CommunityId,
  pub person_id: PersonId,
}

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(
  feature = "full",
  derive(Identifiable, Queryable, Selectable, Associations)
)]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::community::Community))
)]
#[cfg_attr(feature = "full", diesel(table_name = community_person_ban))]
#[cfg_attr(feature = "full", diesel(primary_key(person_id, community_id)))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct CommunityPersonBan {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub published: DateTime<Utc>,
  pub expires: Option<DateTime<Utc>>,
}

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = community_person_ban))]
pub struct CommunityPersonBanForm {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub expires: Option<Option<DateTime<Utc>>>,
}

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(
  feature = "full",
  derive(Identifiable, Queryable, Selectable, Associations)
)]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::community::Community))
)]
#[cfg_attr(feature = "full", diesel(table_name = community_follower))]
#[cfg_attr(feature = "full", diesel(primary_key(person_id, community_id)))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct CommunityFollower {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub published: DateTime<Utc>,
  pub pending: bool,
}

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = community_follower))]
pub struct CommunityFollowerForm {
  pub community_id: CommunityId,
  pub person_id: PersonId,
  pub pending: bool,
}
