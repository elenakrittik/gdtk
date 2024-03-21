pub fn is_stable(ver: &versions::Versioning) -> bool {
    match ver {
        versions::Versioning::Ideal(versions::SemVer {
            pre_rel: Some(versions::Release(vec)),
            ..
        })
        | versions::Versioning::General(versions::Version {
            release: Some(versions::Release(vec)),
            ..
        }) => vec.as_slice() == [versions::Chunk::Alphanum("stable".to_owned())],
        _ => false,
    }
}
