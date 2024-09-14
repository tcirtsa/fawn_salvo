// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct MessagesMessageTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct NotificationsNotificationTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct PrivacySettingsMessagePrivacyEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct PrivacySettingsProfileVisibilityEnum;
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Integer,
        post_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        comment -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    follows (follower_id, following_id) {
        follower_id -> Integer,
        following_id -> Integer,
        follow_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    likes (post_id, user_id) {
        post_id -> Integer,
        user_id -> Integer,
        like_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    live_comments (comment_id) {
        comment_id -> Integer,
        session_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        comment -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    live_sessions (session_id) {
        session_id -> Integer,
        user_id -> Nullable<Integer>,
        #[max_length = 255]
        session_title -> Nullable<Varchar>,
        #[max_length = 255]
        stream_url -> Nullable<Varchar>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MessagesMessageTypeEnum;

    messages (message_id) {
        message_id -> Integer,
        sender_id -> Nullable<Integer>,
        receiver_id -> Nullable<Integer>,
        content -> Nullable<Text>,
        #[max_length = 5]
        message_type -> Nullable<MessagesMessageTypeEnum>,
        sent_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NotificationsNotificationTypeEnum;

    notifications (notification_id) {
        notification_id -> Integer,
        user_id -> Nullable<Integer>,
        #[max_length = 7]
        notification_type -> Nullable<NotificationsNotificationTypeEnum>,
        triggered_by -> Nullable<Integer>,
        post_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Integer,
        user_id -> Nullable<Integer>,
        content -> Nullable<Text>,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        #[max_length = 255]
        video_url -> Nullable<Varchar>,
        #[max_length = 255]
        audio_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PrivacySettingsProfileVisibilityEnum;
    use super::sql_types::PrivacySettingsMessagePrivacyEnum;

    privacy_settings (user_id) {
        user_id -> Integer,
        #[max_length = 7]
        profile_visibility -> Nullable<PrivacySettingsProfileVisibilityEnum>,
        #[max_length = 8]
        message_privacy -> Nullable<PrivacySettingsMessagePrivacyEnum>,
    }
}

diesel::table! {
    recommendations (recommendation_id) {
        recommendation_id -> Integer,
        user_id -> Nullable<Integer>,
        post_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    search_history (search_id) {
        search_id -> Integer,
        user_id -> Nullable<Integer>,
        #[max_length = 255]
        search_query -> Nullable<Varchar>,
        search_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    short_videos (video_id) {
        video_id -> Integer,
        user_id -> Nullable<Integer>,
        #[max_length = 255]
        video_url -> Nullable<Varchar>,
        #[max_length = 255]
        thumbnail_url -> Nullable<Varchar>,
        duration -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 15]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    voice_chat_sessions (session_id) {
        session_id -> Integer,
        user1_id -> Nullable<Integer>,
        user2_id -> Nullable<Integer>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    voice_messages (message_id) {
        message_id -> Integer,
        sender_id -> Nullable<Integer>,
        receiver_id -> Nullable<Integer>,
        #[max_length = 255]
        audio_url -> Nullable<Varchar>,
        duration -> Nullable<Integer>,
        sent_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    follows,
    likes,
    live_comments,
    live_sessions,
    messages,
    notifications,
    posts,
    privacy_settings,
    recommendations,
    search_history,
    short_videos,
    users,
    voice_chat_sessions,
    voice_messages,
);
