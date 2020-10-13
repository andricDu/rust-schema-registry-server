table! {
    schemas (id, version) {
        id -> Int4,
        version -> Int4,
        subject -> Varchar,
        format -> Varchar,
        definition -> Text,
    }
}
