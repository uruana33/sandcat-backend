/// / decode message content by content type
/// / the content is candidate, when message type is Candidate
/// / the content is sustain, when message type is Hangup
/// / the content is String::to_vec(), when message type is SingleMsg/GroupMsg
/// / other message type, the content is bincode::serialize(&T)
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// must have
    #[prost(string, tag = "1")]
    pub send_id: ::prost::alloc::string::String,
    /// must have
    #[prost(string, tag = "2")]
    pub receiver_id: ::prost::alloc::string::String,
    /// must have
    #[prost(string, tag = "3")]
    pub local_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub server_id: ::prost::alloc::string::String,
    /// timestamp
    #[prost(int64, tag = "5")]
    pub create_time: i64,
    #[prost(int64, tag = "6")]
    pub send_time: i64,
    #[prost(int64, tag = "7")]
    pub seq: i64,
    /// is there necessary to cary the user's avatar and nickname?
    #[prost(enumeration = "MsgType", tag = "8")]
    pub msg_type: i32,
    #[prost(enumeration = "ContentType", tag = "9")]
    pub content_type: i32,
    #[prost(bytes = "vec", tag = "10")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "11")]
    pub is_read: bool,
    #[prost(string, optional, tag = "12")]
    pub sdp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub sdp_mid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "14")]
    pub sdp_m_index: ::core::option::Option<i32>,
    #[prost(string, tag = "15")]
    pub group_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRead {
    #[prost(string, tag = "1")]
    pub msg_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub seq: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candidate {
    #[prost(string, tag = "1")]
    pub candidate: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub sdp_mid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub sdp_m_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgreeSingleCall {
    #[prost(string, tag = "1")]
    pub sdp: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleCallInvite {
    #[prost(enumeration = "SingleCallInviteType", tag = "1")]
    pub invite_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleCallInviteAnswer {
    #[prost(bool, tag = "1")]
    pub agree: bool,
    #[prost(enumeration = "SingleCallInviteType", tag = "2")]
    pub invite_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleCallInviteNotAnswer {
    #[prost(enumeration = "SingleCallInviteType", tag = "1")]
    pub invite_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleCallInviteCancel {
    #[prost(enumeration = "SingleCallInviteType", tag = "2")]
    pub invite_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleCallOffer {
    #[prost(string, tag = "1")]
    pub sdp: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hangup {
    #[prost(enumeration = "SingleCallInviteType", tag = "1")]
    pub invite_type: i32,
    #[prost(int64, tag = "2")]
    pub sustain: i64,
}
/// / use to send single message or group message;
/// / message ws is used to connect the client by websocket;
/// / and it receive message from clients; then send message to mq;
/// / so only provide the send message function for other rpc service;
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Single {
    /// message content
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// message type
    #[prost(enumeration = "ContentType", tag = "3")]
    pub content_type: i32,
}
/// / user and group id
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAndGroupId {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
}
/// / group invitation include group information and group member information
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInvitation {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<GroupInfo>,
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
}
/// / group information also related to database
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub announcement: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub create_time: i64,
    #[prost(int64, tag = "8")]
    pub update_time: i64,
}
/// / group member information also related to database table group_members
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    #[prost(int32, tag = "1")]
    pub age: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub joined_at: i64,
    #[prost(string, optional, tag = "7")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "8")]
    pub gender: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub is_friend: bool,
    #[prost(string, optional, tag = "10")]
    pub remark: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "11")]
    pub signature: ::prost::alloc::string::String,
}
/// / create group object
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreate {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub members_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteNew {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUpdate {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub announcement: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub update_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    #[serde(skip_serializing)]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub gender: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub age: i32,
    #[prost(string, optional, tag = "8")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "12")]
    pub birthday: ::core::option::Option<i64>,
    #[prost(int64, tag = "13")]
    pub create_time: i64,
    #[prost(int64, tag = "14")]
    pub update_time: i64,
    #[prost(string, tag = "15")]
    #[serde(skip_serializing)]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserWithMatchType {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub gender: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub age: i32,
    #[prost(string, optional, tag = "7")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "9")]
    pub birthday: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "10")]
    pub match_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "11")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Friendship {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub friend_id: ::prost::alloc::string::String,
    #[prost(enumeration = "FriendshipStatus", tag = "4")]
    pub status: i32,
    #[prost(string, optional, tag = "5")]
    pub apply_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub req_remark: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub resp_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub resp_remark: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "9")]
    pub source: ::prost::alloc::string::String,
    #[prost(int64, tag = "10")]
    pub create_time: i64,
    #[prost(int64, tag = "11")]
    pub accept_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendshipWithUser {
    #[prost(string, tag = "1")]
    pub fs_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub gender: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub age: i32,
    #[prost(string, optional, tag = "7")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "FriendshipStatus", tag = "8")]
    pub status: i32,
    #[prost(string, optional, tag = "9")]
    pub apply_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "10")]
    pub source: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub create_time: i64,
    #[prost(string, tag = "12")]
    pub account: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Friend {
    #[prost(string, tag = "1")]
    pub fs_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub gender: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub age: i32,
    #[prost(string, optional, tag = "6")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "FriendshipStatus", tag = "7")]
    pub status: i32,
    #[prost(string, optional, tag = "8")]
    pub hello: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub remark: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "10")]
    pub source: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub accept_time: i64,
    #[prost(string, tag = "12")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub friend_id: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub signature: ::prost::alloc::string::String,
    #[prost(int64, tag = "15")]
    pub create_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsCreateRequest {
    #[prost(message, optional, tag = "1")]
    pub fs_create: ::core::option::Option<FsCreate>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsCreate {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub friend_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub apply_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub req_remark: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsCreateResponse {
    #[prost(message, optional, tag = "1")]
    pub fs_req: ::core::option::Option<FriendshipWithUser>,
    #[prost(message, optional, tag = "2")]
    pub fs_send: ::core::option::Option<FriendshipWithUser>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsAgreeRequest {
    #[prost(message, optional, tag = "1")]
    pub fs_reply: ::core::option::Option<AgreeReply>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsAgreeResponse {
    #[prost(message, optional, tag = "1")]
    pub req: ::core::option::Option<Friend>,
    #[prost(message, optional, tag = "2")]
    pub send: ::core::option::Option<Friend>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRemarkRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub friend_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub remark: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRemarkResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgreeReply {
    #[prost(string, tag = "1")]
    pub fs_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub resp_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub resp_remark: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsListRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendListRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
/// / only for update friend apply request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsUpdate {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub apply_msg: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub req_remark: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsUpdateRequest {
    #[prost(message, optional, tag = "1")]
    pub fs_update: ::core::option::Option<FsUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsListResponse {
    #[prost(message, repeated, tag = "1")]
    pub friendships: ::prost::alloc::vec::Vec<FriendshipWithUser>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendListResponse {
    #[prost(message, repeated, tag = "1")]
    pub friends: ::prost::alloc::vec::Vec<Friend>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteNewRequest {
    #[prost(message, optional, tag = "1")]
    pub group_invite: ::core::option::Option<GroupInviteNew>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteNewResp {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUpdateRequest {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteRequest {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeleteResponse {
    ///   repeated string members_id = 1;
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupInfo>,
}
///   repeated string members_id = 1;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberExitResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMembersIdRequest {
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMembersIdResponse {
    #[prost(string, repeated, tag = "1")]
    pub members_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pattern: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUserResponse {
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<UserWithMatchType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyPwdRequest {
    /// / could be account, email or phone number
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyPwdResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMsgRequest {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Msg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendGroupMsgRequest {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Msg>,
    #[prost(string, repeated, tag = "2")]
    pub members_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMsgResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgResponse {
    #[prost(string, tag = "1")]
    pub local_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub server_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub send_time: i64,
    #[prost(string, tag = "4")]
    pub err: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveMessageRequest {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Msg>,
    #[prost(bool, tag = "2")]
    pub need_to_history: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveMessageResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveGroupMsgRequest {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Msg>,
    #[prost(bool, tag = "2")]
    pub need_to_history: bool,
    #[prost(string, repeated, tag = "3")]
    pub members_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveGroupMsgResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDbMsgRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub start: i64,
    #[prost(int64, tag = "3")]
    pub end: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMsgResp {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<Msg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateRequest {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<GroupCreate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateResponse {
    #[prost(message, optional, tag = "1")]
    pub invitation: ::core::option::Option<GroupInvitation>,
}
/// / message content type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    Default = 0,
    Text = 1,
    Image = 2,
    Video = 3,
    Audio = 4,
    File = 5,
    Emoji = 6,
    VideoCall = 7,
    AudioCall = 8,
    Error = 9,
}
impl ContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentType::Default => "Default",
            ContentType::Text => "Text",
            ContentType::Image => "Image",
            ContentType::Video => "Video",
            ContentType::Audio => "Audio",
            ContentType::File => "File",
            ContentType::Emoji => "Emoji",
            ContentType::VideoCall => "VideoCall",
            ContentType::AudioCall => "AudioCall",
            ContentType::Error => "Error",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Default" => Some(Self::Default),
            "Text" => Some(Self::Text),
            "Image" => Some(Self::Image),
            "Video" => Some(Self::Video),
            "Audio" => Some(Self::Audio),
            "File" => Some(Self::File),
            "Emoji" => Some(Self::Emoji),
            "VideoCall" => Some(Self::VideoCall),
            "AudioCall" => Some(Self::AudioCall),
            "Error" => Some(Self::Error),
            _ => None,
        }
    }
}
/// / friendship status
#[derive(
    sqlx::Type, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
pub enum FriendshipStatus {
    /// / default status
    Default = 0,
    Pending = 1,
    Accepted = 2,
    Rejected = 3,
    /// / blacklist
    Blacked = 4,
    Canceled = 5,
    Deleted = 6,
}
impl FriendshipStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FriendshipStatus::Default => "FriendshipStatusDefault",
            FriendshipStatus::Pending => "Pending",
            FriendshipStatus::Accepted => "Accepted",
            FriendshipStatus::Rejected => "Rejected",
            FriendshipStatus::Blacked => "Blacked",
            FriendshipStatus::Canceled => "Canceled",
            FriendshipStatus::Deleted => "Deleted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FriendshipStatusDefault" => Some(Self::Default),
            "Pending" => Some(Self::Pending),
            "Accepted" => Some(Self::Accepted),
            "Rejected" => Some(Self::Rejected),
            "Blacked" => Some(Self::Blacked),
            "Canceled" => Some(Self::Canceled),
            "Deleted" => Some(Self::Deleted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    SingleMsg = 0,
    GroupMsg = 1,
    GroupInvitation = 2,
    GroupInviteNew = 3,
    GroupMemberExit = 4,
    GroupDismiss = 5,
    GroupDismissOrExitReceived = 6,
    GroupInvitationReceived = 7,
    GroupUpdate = 8,
    FriendApplyReq = 9,
    FriendApplyResp = 10,
    SingleCallInvite = 11,
    RejectSingleCall = 12,
    AgreeSingleCall = 13,
    SingleCallInviteNotAnswer = 14,
    SingleCallInviteCancel = 15,
    SingleCallOffer = 16,
    Hangup = 17,
    ConnectSingleCall = 18,
    Candidate = 19,
    Read = 20,
    MsgRecResp = 21,
    Notification = 22,
    Service = 23,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::SingleMsg => "MsgTypeSingleMsg",
            MsgType::GroupMsg => "MsgTypeGroupMsg",
            MsgType::GroupInvitation => "MsgTypeGroupInvitation",
            MsgType::GroupInviteNew => "MsgTypeGroupInviteNew",
            MsgType::GroupMemberExit => "MsgTypeGroupMemberExit",
            MsgType::GroupDismiss => "MsgTypeGroupDismiss",
            MsgType::GroupDismissOrExitReceived => "MsgTypeGroupDismissOrExitReceived",
            MsgType::GroupInvitationReceived => "MsgTypeGroupInvitationReceived",
            MsgType::GroupUpdate => "MsgTypeGroupUpdate",
            MsgType::FriendApplyReq => "MsgTypeFriendApplyReq",
            MsgType::FriendApplyResp => "MsgTypeFriendApplyResp",
            MsgType::SingleCallInvite => "MsgTypeSingleCallInvite",
            MsgType::RejectSingleCall => "MsgTypeRejectSingleCall",
            MsgType::AgreeSingleCall => "MsgTypeAgreeSingleCall",
            MsgType::SingleCallInviteNotAnswer => "MsgTypeSingleCallInviteNotAnswer",
            MsgType::SingleCallInviteCancel => "MsgTypeSingleCallInviteCancel",
            MsgType::SingleCallOffer => "MsgTypeSingleCallOffer",
            MsgType::Hangup => "MsgTypeHangup",
            MsgType::ConnectSingleCall => "MsgTypeConnectSingleCall",
            MsgType::Candidate => "MsgTypeCandidate",
            MsgType::Read => "MsgTypeRead",
            MsgType::MsgRecResp => "MsgTypeMsgRecResp",
            MsgType::Notification => "MsgTypeNotification",
            MsgType::Service => "MsgTypeService",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MsgTypeSingleMsg" => Some(Self::SingleMsg),
            "MsgTypeGroupMsg" => Some(Self::GroupMsg),
            "MsgTypeGroupInvitation" => Some(Self::GroupInvitation),
            "MsgTypeGroupInviteNew" => Some(Self::GroupInviteNew),
            "MsgTypeGroupMemberExit" => Some(Self::GroupMemberExit),
            "MsgTypeGroupDismiss" => Some(Self::GroupDismiss),
            "MsgTypeGroupDismissOrExitReceived" => Some(Self::GroupDismissOrExitReceived),
            "MsgTypeGroupInvitationReceived" => Some(Self::GroupInvitationReceived),
            "MsgTypeGroupUpdate" => Some(Self::GroupUpdate),
            "MsgTypeFriendApplyReq" => Some(Self::FriendApplyReq),
            "MsgTypeFriendApplyResp" => Some(Self::FriendApplyResp),
            "MsgTypeSingleCallInvite" => Some(Self::SingleCallInvite),
            "MsgTypeRejectSingleCall" => Some(Self::RejectSingleCall),
            "MsgTypeAgreeSingleCall" => Some(Self::AgreeSingleCall),
            "MsgTypeSingleCallInviteNotAnswer" => Some(Self::SingleCallInviteNotAnswer),
            "MsgTypeSingleCallInviteCancel" => Some(Self::SingleCallInviteCancel),
            "MsgTypeSingleCallOffer" => Some(Self::SingleCallOffer),
            "MsgTypeHangup" => Some(Self::Hangup),
            "MsgTypeConnectSingleCall" => Some(Self::ConnectSingleCall),
            "MsgTypeCandidate" => Some(Self::Candidate),
            "MsgTypeRead" => Some(Self::Read),
            "MsgTypeMsgRecResp" => Some(Self::MsgRecResp),
            "MsgTypeNotification" => Some(Self::Notification),
            "MsgTypeService" => Some(Self::Service),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SingleCallInviteType {
    SingleAudio = 0,
    SingleVideo = 1,
}
impl SingleCallInviteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SingleCallInviteType::SingleAudio => "SingleAudio",
            SingleCallInviteType::SingleVideo => "SingleVideo",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SingleAudio" => Some(Self::SingleAudio),
            "SingleVideo" => Some(Self::SingleVideo),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod msg_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// send message through rpc
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.MsgService/SendMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.MsgService", "SendMessage"));
            self.inner.unary(req, path, codec).await
        }
        /// send single message to user by websocket
        pub async fn send_msg_to_user(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.MsgService/SendMsgToUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.MsgService", "SendMsgToUser"));
            self.inner.unary(req, path, codec).await
        }
        /// send group message to user by websocket
        pub async fn send_group_msg_to_user(
            &mut self,
            request: impl tonic::IntoRequest<super::SendGroupMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/message.MsgService/SendGroupMsgToUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.MsgService", "SendGroupMsgToUser"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod chat_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// / chat service, receive message then generate message id and send message to mq;
    /// / response operation result;
    #[derive(Debug, Clone)]
    pub struct ChatServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChatServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChatServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ChatServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ChatServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn send_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.ChatService/SendMsg");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.ChatService", "SendMsg"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod db_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// / db interface think about if it is necessary to put api interface together.
    #[derive(Debug, Clone)]
    pub struct DbServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DbServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DbServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DbServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DbServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// / save message to postgres and mongodb
        /// / use same table and collection to save the single message and group message
        pub async fn save_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveMessageRequest>,
        ) -> std::result::Result<tonic::Response<super::SaveMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/SaveMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "SaveMessage"));
            self.inner.unary(req, path, codec).await
        }
        /// / save group message to postgres and mongodb
        pub async fn save_group_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveGroupMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SaveGroupMsgResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/SaveGroupMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "SaveGroupMessage"));
            self.inner.unary(req, path, codec).await
        }
        /// / query message from mongodb by start seq to end seq
        pub async fn get_msg_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDbMsgRequest>,
        ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Msg>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GetMsgStream");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GetMsgStream"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDbMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::GetMsgResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GetMessages");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GetMessages"));
            self.inner.unary(req, path, codec).await
        }
        /// / create group
        pub async fn group_create(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupCreateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GroupCreate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GroupCreate"));
            self.inner.unary(req, path, codec).await
        }
        /// / member invites new member
        pub async fn group_invite_new(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupInviteNewRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupInviteNewResp>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GroupInviteNew");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GroupInviteNew"));
            self.inner.unary(req, path, codec).await
        }
        /// / update group
        pub async fn group_update(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupUpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupUpdateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GroupUpdate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GroupUpdate"));
            self.inner.unary(req, path, codec).await
        }
        /// / delete group
        pub async fn group_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupDeleteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GroupDelete");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GroupDelete"));
            self.inner.unary(req, path, codec).await
        }
        /// / member exit
        pub async fn group_member_exit(
            &mut self,
            request: impl tonic::IntoRequest<super::UserAndGroupId>,
        ) -> std::result::Result<tonic::Response<super::GroupMemberExitResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GroupMemberExit");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GroupMemberExit"));
            self.inner.unary(req, path, codec).await
        }
        /// / query group members id
        pub async fn group_members_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupMembersIdRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupMembersIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GroupMembersId");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GroupMembersId"));
            self.inner.unary(req, path, codec).await
        }
        /// / create user
        pub async fn create_user(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateUserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/CreateUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "CreateUser"));
            self.inner.unary(req, path, codec).await
        }
        /// / get user by id
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::GetUserResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GetUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GetUser"));
            self.inner.unary(req, path, codec).await
        }
        /// / update user
        pub async fn update_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateUserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/UpdateUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "UpdateUser"));
            self.inner.unary(req, path, codec).await
        }
        /// / search user
        pub async fn search_user(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchUserRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchUserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/SearchUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "SearchUser"));
            self.inner.unary(req, path, codec).await
        }
        /// / verify password
        pub async fn verify_password(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyPwdRequest>,
        ) -> std::result::Result<tonic::Response<super::VerifyPwdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/VerifyPassword");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "VerifyPassword"));
            self.inner.unary(req, path, codec).await
        }
        /// / create friendship
        pub async fn create_friendship(
            &mut self,
            request: impl tonic::IntoRequest<super::FsCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::FsCreateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/CreateFriendship");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "CreateFriendship"));
            self.inner.unary(req, path, codec).await
        }
        /// / reply friendship: agree, reject
        pub async fn agree_friendship(
            &mut self,
            request: impl tonic::IntoRequest<super::FsAgreeRequest>,
        ) -> std::result::Result<tonic::Response<super::FsAgreeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/AgreeFriendship");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "AgreeFriendship"));
            self.inner.unary(req, path, codec).await
        }
        /// / get friendship list
        pub async fn get_friendship_list(
            &mut self,
            request: impl tonic::IntoRequest<super::FsListRequest>,
        ) -> std::result::Result<tonic::Response<super::FsListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GetFriendshipList");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GetFriendshipList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_friend_list(
            &mut self,
            request: impl tonic::IntoRequest<super::FriendListRequest>,
        ) -> std::result::Result<tonic::Response<super::FriendListResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.DbService/GetFriendList");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "GetFriendList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_friend_remark(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRemarkRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateRemarkResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/message.DbService/UpdateFriendRemark");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.DbService", "UpdateFriendRemark"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod push_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PushServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PushServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PushServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PushServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PushServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// / push single message to message gateway
        pub async fn push_single_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.PushService/PushSingleMsg");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.PushService", "PushSingleMsg"));
            self.inner.unary(req, path, codec).await
        }
        /// / push group message to message gateway
        pub async fn push_group_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::SendGroupMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.PushService/PushGroupMsg");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.PushService", "PushGroupMsg"));
            self.inner.unary(req, path, codec).await
        }
        /// / push other message to message gateway, need to think about it
        pub async fn push_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.PushService/PushMsg");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.PushService", "PushMsg"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod msg_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServiceServer.
    #[async_trait]
    pub trait MsgService: Send + Sync + 'static {
        /// send message through rpc
        async fn send_message(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
        /// send single message to user by websocket
        async fn send_msg_to_user(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
        /// send group message to user by websocket
        async fn send_group_msg_to_user(
            &self,
            request: tonic::Request<super::SendGroupMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MsgServiceServer<T: MsgService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MsgService> MsgServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServiceServer<T>
    where
        T: MsgService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/message.MsgService/SendMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageSvc<T: MsgService>(pub Arc<T>);
                    impl<T: MsgService> tonic::server::UnaryService<super::SendMsgRequest> for SendMessageSvc<T> {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MsgService>::send_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.MsgService/SendMsgToUser" => {
                    #[allow(non_camel_case_types)]
                    struct SendMsgToUserSvc<T: MsgService>(pub Arc<T>);
                    impl<T: MsgService> tonic::server::UnaryService<super::SendMsgRequest> for SendMsgToUserSvc<T> {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MsgService>::send_msg_to_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMsgToUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.MsgService/SendGroupMsgToUser" => {
                    #[allow(non_camel_case_types)]
                    struct SendGroupMsgToUserSvc<T: MsgService>(pub Arc<T>);
                    impl<T: MsgService> tonic::server::UnaryService<super::SendGroupMsgRequest>
                        for SendGroupMsgToUserSvc<T>
                    {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendGroupMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MsgService>::send_group_msg_to_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendGroupMsgToUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: MsgService> Clone for MsgServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: MsgService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MsgService> tonic::server::NamedService for MsgServiceServer<T> {
        const NAME: &'static str = "message.MsgService";
    }
}
/// Generated server implementations.
pub mod chat_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ChatServiceServer.
    #[async_trait]
    pub trait ChatService: Send + Sync + 'static {
        async fn send_msg(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::MsgResponse>, tonic::Status>;
    }
    /// / chat service, receive message then generate message id and send message to mq;
    /// / response operation result;
    #[derive(Debug)]
    pub struct ChatServiceServer<T: ChatService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChatService> ChatServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChatServiceServer<T>
    where
        T: ChatService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/message.ChatService/SendMsg" => {
                    #[allow(non_camel_case_types)]
                    struct SendMsgSvc<T: ChatService>(pub Arc<T>);
                    impl<T: ChatService> tonic::server::UnaryService<super::SendMsgRequest> for SendMsgSvc<T> {
                        type Response = super::MsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as ChatService>::send_msg(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMsgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ChatService> Clone for ChatServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ChatService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChatService> tonic::server::NamedService for ChatServiceServer<T> {
        const NAME: &'static str = "message.ChatService";
    }
}
/// Generated server implementations.
pub mod db_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DbServiceServer.
    #[async_trait]
    pub trait DbService: Send + Sync + 'static {
        /// / save message to postgres and mongodb
        /// / use same table and collection to save the single message and group message
        async fn save_message(
            &self,
            request: tonic::Request<super::SaveMessageRequest>,
        ) -> std::result::Result<tonic::Response<super::SaveMessageResponse>, tonic::Status>;
        /// / save group message to postgres and mongodb
        async fn save_group_message(
            &self,
            request: tonic::Request<super::SaveGroupMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SaveGroupMsgResponse>, tonic::Status>;
        /// Server streaming response type for the GetMsgStream method.
        type GetMsgStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Msg, tonic::Status>,
            > + Send
            + 'static;
        /// / query message from mongodb by start seq to end seq
        async fn get_msg_stream(
            &self,
            request: tonic::Request<super::GetDbMsgRequest>,
        ) -> std::result::Result<tonic::Response<Self::GetMsgStreamStream>, tonic::Status>;
        async fn get_messages(
            &self,
            request: tonic::Request<super::GetDbMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::GetMsgResp>, tonic::Status>;
        /// / create group
        async fn group_create(
            &self,
            request: tonic::Request<super::GroupCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupCreateResponse>, tonic::Status>;
        /// / member invites new member
        async fn group_invite_new(
            &self,
            request: tonic::Request<super::GroupInviteNewRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupInviteNewResp>, tonic::Status>;
        /// / update group
        async fn group_update(
            &self,
            request: tonic::Request<super::GroupUpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupUpdateResponse>, tonic::Status>;
        /// / delete group
        async fn group_delete(
            &self,
            request: tonic::Request<super::GroupDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupDeleteResponse>, tonic::Status>;
        /// / member exit
        async fn group_member_exit(
            &self,
            request: tonic::Request<super::UserAndGroupId>,
        ) -> std::result::Result<tonic::Response<super::GroupMemberExitResponse>, tonic::Status>;
        /// / query group members id
        async fn group_members_id(
            &self,
            request: tonic::Request<super::GroupMembersIdRequest>,
        ) -> std::result::Result<tonic::Response<super::GroupMembersIdResponse>, tonic::Status>;
        /// / create user
        async fn create_user(
            &self,
            request: tonic::Request<super::CreateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateUserResponse>, tonic::Status>;
        /// / get user by id
        async fn get_user(
            &self,
            request: tonic::Request<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::GetUserResponse>, tonic::Status>;
        /// / update user
        async fn update_user(
            &self,
            request: tonic::Request<super::UpdateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateUserResponse>, tonic::Status>;
        /// / search user
        async fn search_user(
            &self,
            request: tonic::Request<super::SearchUserRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchUserResponse>, tonic::Status>;
        /// / verify password
        async fn verify_password(
            &self,
            request: tonic::Request<super::VerifyPwdRequest>,
        ) -> std::result::Result<tonic::Response<super::VerifyPwdResponse>, tonic::Status>;
        /// / create friendship
        async fn create_friendship(
            &self,
            request: tonic::Request<super::FsCreateRequest>,
        ) -> std::result::Result<tonic::Response<super::FsCreateResponse>, tonic::Status>;
        /// / reply friendship: agree, reject
        async fn agree_friendship(
            &self,
            request: tonic::Request<super::FsAgreeRequest>,
        ) -> std::result::Result<tonic::Response<super::FsAgreeResponse>, tonic::Status>;
        /// / get friendship list
        async fn get_friendship_list(
            &self,
            request: tonic::Request<super::FsListRequest>,
        ) -> std::result::Result<tonic::Response<super::FsListResponse>, tonic::Status>;
        async fn get_friend_list(
            &self,
            request: tonic::Request<super::FriendListRequest>,
        ) -> std::result::Result<tonic::Response<super::FriendListResponse>, tonic::Status>;
        async fn update_friend_remark(
            &self,
            request: tonic::Request<super::UpdateRemarkRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateRemarkResponse>, tonic::Status>;
    }
    /// / db interface think about if it is necessary to put api interface together.
    #[derive(Debug)]
    pub struct DbServiceServer<T: DbService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DbService> DbServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DbServiceServer<T>
    where
        T: DbService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/message.DbService/SaveMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SaveMessageSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::SaveMessageRequest> for SaveMessageSvc<T> {
                        type Response = super::SaveMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::save_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/SaveGroupMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SaveGroupMessageSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::SaveGroupMsgRequest>
                        for SaveGroupMessageSvc<T>
                    {
                        type Response = super::SaveGroupMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveGroupMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::save_group_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveGroupMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GetMsgStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetMsgStreamSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::ServerStreamingService<super::GetDbMsgRequest>
                        for GetMsgStreamSvc<T>
                    {
                        type Response = super::Msg;
                        type ResponseStream = T::GetMsgStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDbMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::get_msg_stream(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMsgStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GetMessages" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessagesSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GetDbMsgRequest> for GetMessagesSvc<T> {
                        type Response = super::GetMsgResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDbMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::get_messages(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GroupCreate" => {
                    #[allow(non_camel_case_types)]
                    struct GroupCreateSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GroupCreateRequest> for GroupCreateSvc<T> {
                        type Response = super::GroupCreateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupCreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::group_create(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GroupInviteNew" => {
                    #[allow(non_camel_case_types)]
                    struct GroupInviteNewSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GroupInviteNewRequest>
                        for GroupInviteNewSvc<T>
                    {
                        type Response = super::GroupInviteNewResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupInviteNewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::group_invite_new(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupInviteNewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GroupUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct GroupUpdateSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GroupUpdateRequest> for GroupUpdateSvc<T> {
                        type Response = super::GroupUpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupUpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::group_update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GroupDelete" => {
                    #[allow(non_camel_case_types)]
                    struct GroupDeleteSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GroupDeleteRequest> for GroupDeleteSvc<T> {
                        type Response = super::GroupDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::group_delete(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GroupMemberExit" => {
                    #[allow(non_camel_case_types)]
                    struct GroupMemberExitSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::UserAndGroupId> for GroupMemberExitSvc<T> {
                        type Response = super::GroupMemberExitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserAndGroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::group_member_exit(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupMemberExitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GroupMembersId" => {
                    #[allow(non_camel_case_types)]
                    struct GroupMembersIdSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GroupMembersIdRequest>
                        for GroupMembersIdSvc<T>
                    {
                        type Response = super::GroupMembersIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupMembersIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::group_members_id(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GroupMembersIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/CreateUser" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::CreateUserRequest> for CreateUserSvc<T> {
                        type Response = super::CreateUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as DbService>::create_user(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::GetUserRequest> for GetUserSvc<T> {
                        type Response = super::GetUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as DbService>::get_user(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/UpdateUser" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::UpdateUserRequest> for UpdateUserSvc<T> {
                        type Response = super::UpdateUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as DbService>::update_user(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/SearchUser" => {
                    #[allow(non_camel_case_types)]
                    struct SearchUserSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::SearchUserRequest> for SearchUserSvc<T> {
                        type Response = super::SearchUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as DbService>::search_user(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/VerifyPassword" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyPasswordSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::VerifyPwdRequest> for VerifyPasswordSvc<T> {
                        type Response = super::VerifyPwdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyPwdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::verify_password(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/CreateFriendship" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFriendshipSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::FsCreateRequest> for CreateFriendshipSvc<T> {
                        type Response = super::FsCreateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FsCreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::create_friendship(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFriendshipSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/AgreeFriendship" => {
                    #[allow(non_camel_case_types)]
                    struct AgreeFriendshipSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::FsAgreeRequest> for AgreeFriendshipSvc<T> {
                        type Response = super::FsAgreeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FsAgreeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::agree_friendship(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AgreeFriendshipSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GetFriendshipList" => {
                    #[allow(non_camel_case_types)]
                    struct GetFriendshipListSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::FsListRequest> for GetFriendshipListSvc<T> {
                        type Response = super::FsListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FsListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::get_friendship_list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFriendshipListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/GetFriendList" => {
                    #[allow(non_camel_case_types)]
                    struct GetFriendListSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::FriendListRequest> for GetFriendListSvc<T> {
                        type Response = super::FriendListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FriendListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::get_friend_list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFriendListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.DbService/UpdateFriendRemark" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFriendRemarkSvc<T: DbService>(pub Arc<T>);
                    impl<T: DbService> tonic::server::UnaryService<super::UpdateRemarkRequest>
                        for UpdateFriendRemarkSvc<T>
                    {
                        type Response = super::UpdateRemarkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRemarkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DbService>::update_friend_remark(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateFriendRemarkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DbService> Clone for DbServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DbService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DbService> tonic::server::NamedService for DbServiceServer<T> {
        const NAME: &'static str = "message.DbService";
    }
}
/// Generated server implementations.
pub mod push_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PushServiceServer.
    #[async_trait]
    pub trait PushService: Send + Sync + 'static {
        /// / push single message to message gateway
        async fn push_single_msg(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
        /// / push group message to message gateway
        async fn push_group_msg(
            &self,
            request: tonic::Request<super::SendGroupMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
        /// / push other message to message gateway, need to think about it
        async fn push_msg(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PushServiceServer<T: PushService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PushService> PushServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PushServiceServer<T>
    where
        T: PushService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/message.PushService/PushSingleMsg" => {
                    #[allow(non_camel_case_types)]
                    struct PushSingleMsgSvc<T: PushService>(pub Arc<T>);
                    impl<T: PushService> tonic::server::UnaryService<super::SendMsgRequest> for PushSingleMsgSvc<T> {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PushService>::push_single_msg(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PushSingleMsgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.PushService/PushGroupMsg" => {
                    #[allow(non_camel_case_types)]
                    struct PushGroupMsgSvc<T: PushService>(pub Arc<T>);
                    impl<T: PushService> tonic::server::UnaryService<super::SendGroupMsgRequest>
                        for PushGroupMsgSvc<T>
                    {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendGroupMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PushService>::push_group_msg(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PushGroupMsgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.PushService/PushMsg" => {
                    #[allow(non_camel_case_types)]
                    struct PushMsgSvc<T: PushService>(pub Arc<T>);
                    impl<T: PushService> tonic::server::UnaryService<super::SendMsgRequest> for PushMsgSvc<T> {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as PushService>::push_msg(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PushMsgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PushService> Clone for PushServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: PushService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PushService> tonic::server::NamedService for PushServiceServer<T> {
        const NAME: &'static str = "message.PushService";
    }
}