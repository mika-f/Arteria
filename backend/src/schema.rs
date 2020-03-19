table! {
    dependencies (id) {
        id -> Bigint,
        instance_id -> Bigint,
        name_with_version -> Varchar,
    }
}

table! {
    executors (id) {
        id -> Integer,
        name -> Varchar,
        image -> Varchar,
        tag -> Varchar,
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
        executor_id -> Integer,
        status -> crate::models::InstanceStatusMapping,
        result -> Nullable<Text>,
    }
}

joinable!(dependencies -> instances (instance_id));
joinable!(files -> instances (instance_id));
joinable!(instances -> executors (executor_id));

allow_tables_to_appear_in_same_query!(dependencies, executors, files, instances,);
