diesel::table! {
    users  {
        id -> Integer,
        name -> Text,
        publicKey -> Text,
        privateKey -> Text,
    }
}
