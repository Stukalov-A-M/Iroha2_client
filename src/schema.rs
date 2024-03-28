
diesel::table! {
    users (name) {
        name -> Text,
        publicKey -> Text,
        privateKey -> Text,
    }
}