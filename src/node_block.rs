// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net
// Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3,
// depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project
// generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.
// This, along with the
// Licenses can be found in the root directory of this project at LICENSE,
// COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network
// Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES
// OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions
// and limitations
// relating to use of the SAFE Network Software.

use maidsafe_utilities::serialisation;
use sodiumoxide::crypto::sign::{Signature, PublicKey, SecretKey};
use sodiumoxide::crypto;
use block_identifier::BlockIdentifier;
use error::Error;

/// If data block then this is sent by any group member when data is `Put`, `Post` or `Delete`.
/// If this is a link then it is sent with a `churn` event.
/// A `Link` is a nodeblock that each member must send each other in times of churn.
/// These will not accumulate but be `ManagedNode`  to `ManagedNode` messages in the routing layer
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct NodeBlock {
    identifier: BlockIdentifier,
    proof: (PublicKey, Signature),
}

impl NodeBlock {
    /// Create a Block (used by nodes in network to send to holders of `DataChains`)
    pub fn new(pub_key: &PublicKey,
               secret_key: &SecretKey,
               data_identifier: BlockIdentifier)
               -> Result<NodeBlock, Error> {
        let signature =
            crypto::sign::sign_detached(&try!(serialisation::serialise(&data_identifier))[..],
                                        secret_key);

        Ok(NodeBlock {
            identifier: data_identifier,
            proof: (pub_key.clone(), signature),
        })

    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use block_identifier::BlockIdentifier;
    use sodiumoxide::crypto;
    use sodiumoxide::crypto::hash::sha256;

    #[test]
    fn node_block_comparisons() {
        ::sodiumoxide::init();
        let keys = crypto::sign::gen_keypair();
        let test_data1 = BlockIdentifier::Link(sha256::hash("1".as_bytes()));
        let test_data2 = BlockIdentifier::Link(sha256::hash("1".as_bytes()));
        let test_data3 = BlockIdentifier::ImmutableData(sha256::hash("1".as_bytes()));
        let test_node_data_block1 = NodeBlock::new(&keys.0, &keys.1, test_data1).expect("fail1");
        let test_node_data_block2 = NodeBlock::new(&keys.0, &keys.1, test_data2).expect("fail2");
        let test_node_data_block3 = NodeBlock::new(&keys.0, &keys.1, test_data3).expect("fail3");
        assert_eq!(test_node_data_block1.clone(), test_node_data_block2.clone());
        assert!(test_node_data_block1 != test_node_data_block3.clone());
        assert!(test_node_data_block2 != test_node_data_block3);

    }
}