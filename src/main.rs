use std::time::SystemTime;

use rust_protobuf::pb::{blog, user};

fn main() {
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let user = user::User {
        id: "0x123".to_string(),
        name: "John".to_string(),
        email: "john@example.com".to_string(),
        salary: 1000,
        role: "admin".to_string(),
        created_at: Some(prost_types::Timestamp {
            seconds: t.as_secs() as i64,
            nanos: t.subsec_nanos() as i32,
        }),
    };
    println!("user: {:?}", user);

    let my_blog = blog::Blog {
        id: 100 as i64,
        title: "Hello world".to_string(),
        content: "Hello world".to_string(),
        author: "John".to_string(),
        created_at: Some(prost_types::Timestamp {
            seconds: t.as_secs() as i64,
            nanos: t.subsec_nanos() as i32,
        }),
        updated_at: Some(prost_types::Timestamp {
            seconds: t.as_secs() as i64,
            nanos: t.subsec_nanos() as i32,
        }),
        tags: vec!["rust".to_string(), "protobuf".to_string()],
    };

    println!("blog: {:?}", my_blog);
}
