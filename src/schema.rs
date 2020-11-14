table! {
    items (id) {
        id -> Nullable<Text>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
    }
}
