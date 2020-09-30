fn main() {
    // START

    const SPEED_LIMIT: u32 = 299_792_458;
    println!("{}", SPEED_LIMIT);
    // 299792458

    // END - START

    struct Action<'a> {
        a_type: &'a str,
    }
    let struct_instance = Action { a_type: "ADD" };

    match struct_instance {
        Action { a_type: "ADD" } => {
            println!("struct: found ADD");
        }
        // have to define a catchall case since the struct has infinite possibilities
        Action { a_type: _ } => {
            println!("struct: catchall case");
        }
    }

    enum AType {
        ADD,
        REMOVE,
    }

    let enum_variant = AType::ADD;

    match enum_variant {
        // don't have to provide a catchall case, as long as we cover every possible value for the enum
        AType::ADD => {
            println!("enum: found ADD");
        }
        AType::REMOVE => {
            println!("enum: found REMOVE");
        }
    }

    // END
}
