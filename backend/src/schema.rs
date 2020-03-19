table! {
    dependencies (id) {
        id -> Bigint,
        instance_id -> Bigint,
        name_with_version -> Varchar,
    }
}

table! {
    files (id) {
        id -> Bigint,
        instance_id -> Bigint,
        title -> Varchar,
        content -> Text,
    }
}

table! {
    instances (id) {
        id -> Bigint,
        title -> Varchar,
        version_id -> Integer,
        status -> crate::models::InstanceStatusMapping,
        result -> Nullable<Text>,
    }
}

table! {
    versions (id) {
        id -> Integer,
        name -> Varchar,
        tag -> Varchar,
    }
}

joinable!(dependencies -> instances (instance_id));
joinable!(files -> instances (instance_id));
joinable!(instances -> versions (version_id));

allow_tables_to_appear_in_same_query!(dependencies, files, instances, versions,);
