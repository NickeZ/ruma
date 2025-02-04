//! [POST /_matrix/identity/v2/lookup](https://matrix.org/docs/spec/identity_service/r0.3.0#post-matrix-identity-v2-lookup)

use std::collections::BTreeMap;

use ruma_api::ruma_api;
use ruma_identifiers::UserId;

use crate::lookup::IdentifierHashingAlgorithm;

ruma_api! {
    metadata: {
        description: "Looks up the set of Matrix User IDs which have bound the 3PIDs given, if bindings are available.",
        method: POST,
        name: "lookup_3pid",
        path: "/_matrix/identity/v2/lookup",
        authentication: AccessToken,
        rate_limited: false,
    }

    request: {
        /// The algorithm the client is using to encode the `addresses`. This should be one of the
        /// available options from `/hash_details`.
        pub algorithm: &'a IdentifierHashingAlgorithm,

        /// The pepper from `/hash_details`. This is required even when the `algorithm` does not
        /// make use of it.
        pub pepper: &'a str,

        /// The addresses to look up.
        ///
        /// The format of the entries here depend on the `algorithm` used. Note that queries which
        /// have been incorrectly hashed or formatted will lead to no matches.
        pub addresses: &'a [String],
    }

    response: {
        /// Any applicable mappings of `addresses` to Matrix User IDs.
        ///
        /// Addresses which do not have associations will not be included, which can make this
        /// property be an empty object.
        pub mappings: BTreeMap<String, UserId>,
    }
}

impl<'a> Request<'a> {
    /// Create a `Request` with algorithm, pepper and addresses to loop up.
    pub fn new(
        algorithm: &'a IdentifierHashingAlgorithm,
        pepper: &'a str,
        addresses: &'a [String],
    ) -> Self {
        Self { algorithm, pepper, addresses }
    }
}

impl Response {
    /// Create a `Response` with the BTreeMap which map addresses from the request which were
    /// found to their corresponding User IDs.
    pub fn new(mappings: BTreeMap<String, UserId>) -> Self {
        Self { mappings }
    }
}
