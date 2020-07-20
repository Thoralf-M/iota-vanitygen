# Vanitygen for IOTA Ed25519 addresses

Search addresses with custom char combinations at the beginning. For example search for `Test` to get an address like `TRoLqWAPoij9ARYMcnkRW65jTbWynHt7vokYXqCrUvNP` and the seed for it.

Addresses are base58 encoded, so only the following chars are allowed `1 2 3 4 5 6 7 8 9  A B C D E F G H   J K L M N   P Q R S T U V W X Y Z a b c d e f g h i j k   m n o p q r s t u v w x y z`

`0OlI` are not allowed

Also, the first character can only be one of `a b J K L M N P Q R S T U V W X Y Z`

Run it with `cargo run --release`

Searching for a combination with more than 4 characters can take a long time