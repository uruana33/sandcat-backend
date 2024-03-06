// @generated automatically by Diesel CLI.

diesel::table! {
    friends (id) {
        id -> Varchar,
        friendship_id -> Varchar,
        user_id -> Varchar,
        friend_id -> Varchar,
        #[max_length = 1]
        status -> Bpchar,
        remark -> Nullable<Varchar>,
        hello -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    friendships (id) {
        id -> Varchar,
        user_id -> Varchar,
        friend_id -> Varchar,
        #[max_length = 1]
        status -> Bpchar,
        apply_msg -> Nullable<Varchar>,
        req_remark -> Nullable<Varchar>,
        response_msg -> Nullable<Varchar>,
        res_remark -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        is_delivered -> Bool,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Varchar,
        owner -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        members -> Text,
        avatar -> Text,
        description -> Text,
        announcement -> Text,
        create_time -> Timestamp,
    }
}

diesel::table! {
    messages (msg_id) {
        msg_id -> Varchar,
        msg_type -> Varchar,
        content -> Varchar,
        content_type -> Varchar,
        send_id -> Varchar,
        friend_id -> Varchar,
        is_read -> Bool,
        delivered -> Bool,
        create_time -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        account -> Varchar,
        password -> Varchar,
        avatar -> Varchar,
        gender -> Varchar,
        age -> Int4,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 64]
        email -> Nullable<Varchar>,
        #[max_length = 1024]
        address -> Nullable<Varchar>,
        birthday -> Nullable<Timestamp>,
        create_time -> Timestamp,
        update_time -> Timestamp,
        is_delete -> Bool,
    }
}

diesel::joinable!(groups -> users (owner));

diesel::joinable!(friendships ->users(user_id));
diesel::joinable!(friends ->users(friend_id));
diesel::allow_tables_to_appear_in_same_query!(friends, friendships, groups, messages, users,);
