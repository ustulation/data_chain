- Feature Name: Data Blocks
- Status: proposed
- Type: new feature
- Related components: (data, routing, vaults)
- Start Date: 08-03-2016
- RFC PR: (leave this empty)
- Issue number: (leave this empty)

# Summary

Data blocks are a container that allows large blocks of data to be maintained. These blocks can be
validated by a node on a network, close to the data name, to contain valid data that was
 guaranteed to have been correctly stored onto the network.

# Definitions used

- Decentralised network, A peer to peer network in xor space, using Kadmelia type addressing.
- Hash, a cryptographic one way function that produces a fixed length representation of any input.
- Immutable data, a data type that has a name == hash of it's contents (it is immutable as changing
  the contents creates a new piece of immutable data).
- Structured data, a data type that has a fixed name, but mutable contents.
- GROUP_SIZE, the number of nodes surrounding a network address.
- QUORUM, the number of the GROUP that is considered large enough that a decision is valid. In this
  paper this number is considered a majority (i.e. (GROUP_SIZE / 2) + 1)
- Chain consensus, the fact that QUORUM number of signatories exist in the next link (`DataBlock` as
  described below) that also exist in the previous block.
- Churn event, a change in the group, either by a node leaving or a node joining.


# Abstract

A mechanism to lock data descriptors in containers that may be held on a decentralised network.
Such structures are cryptographically secured in lock step using a consensus of cryptographic
signatures. These signatures are of a certain size GROUP_SIZE (e.g. 12 nodes) with a QUORUM (e.g. 7
nodes) required to be considered valid (much like N of P sharing). In a decentralised network that
has secured groups,these signatures are those closest to the holder of a `DataChain`. The
`DataChain` will have a majority of existing group members if it is republished prior to more than
GROUP_SIZE - QUORUM nodes changing. In this situation, there is a strong cryptographic proof of the
data validity.

When a `DataChain` starts, the first item is probably a `link`. This is a block that uses the
identity of a close group on the network. This `link` has an associated proof that is the
`PublicKey` and a corresponding signature for each node. The `Signature` is the signed `link`
block.  On each `churn` event a new link is created and again signed by all members of the
close_group. This link is the nodes close group as known by all members of that close_group. The
link is the xor result of that close_group.

Data block entries are signed by an ever changing majority of pre-existing nodes.  As the chain
grows, this rolling majority of different signatories can be cryptographically confirmed (via
`links`).  This process continues to the very top of the chain which will contain entries signed by
the majority of the current close group of nodes. This current group of nodes can then
cryptographically validate the entire chain and every data element referred to within it.

A data chain may look like

`link:data:data:data:data:link:link:data:data`

or

`link:link:link:data:link:link:link:data:link`

The `links` maintain group consensus and the data elements should individually validate all data
blocks though the group consensus provided by the preceding `link`.

As groups change and the network grows, or indeed shrinks, many chains held by various nodes will
have a common element. This allows such chains to be cross referenced in order to build a complete
picture of data from the start of the network. In essence, this chain of verifiable data elements
provides a provable sequence of data validity and also the sequence of such data appearing on the
network. It is assumed that a later project using graph analysis can provide analytics that may be
subjected to deep learning algorithms that will improve network security and efficiency.

It is through this basic recondition of chained majority agreements that assures the ability for a
`DataChain` to be validated and therefore allows data to be republished.

The design described below will show a system where node capabilities are amortised across a
network, providing a balance of resources that can be mixed evenly across a network of nodes with
varying capabilities, from mass persistent data storage to node with very little, transient data
storage.

# Motivation

In a fully decentralised network there are many problems to solve, two of these issues can be
thought of as:

1. How to handle transferring large amounts of data to replicant nodes on each churn event.

2. How to allow data to be republished in a secure manner.

Point 2 actually encompasses two large issues in itself. The ability to start a node and make it's
data available is obviously required where we have large amounts of data to maintain. Another large
advantage is the ability for such a network to recover from a full system outage (full network
collapse, worldwide power outage etc.).


# Detailed design

## Data covered by a data chain

Nodes in a decentralised network may have many common groups (log2(n) - 1, actually, where n =
address size in bits). These common groups will each hold exponentially less data than the closest
group. This proposal is aimed at only the closest group to a node. This means any data that can be
validated by a majority of the current nodes close peers will be considered valid to push into the
chain. As a chain will be transferable (with the data) it will not have an identifier of any
particular address. Instead the identifiers for the groups will appear somewhat arbitrary,

In fact the link identifiers to identify the group will not in fact identify any address in
particular, at least in relation to the data space covered by the chain. This seems
counter-intuitive, but is in fact important. What concerns us in this design is that at least all
group members agree on something that they can sign to attest to this group having existed on the
network and in a manner they all agree on. To achieve this we again use `xor` and as described
below the identifier for links is merely the xor of all group members in relation to individual
nodes and not any data item itself.

This will no doubt cause confusion to the reader, but it is assumed that this will become apparent
as the design unfolds below.

## BlockDentifier

A [BlockIdentifier][1] is simple enumeration that represents, either a `Data` item (`structuredData`
or `ImmutableData`).

The other type that can be represented in the `enum` is a `Link`. A `Link` represents a valid group
of nodes that is close to a point in the Xor address space. This point changes with respect to
changing nodes around any node. The representation of the link address in the chain (which is not
representative of the address of the data or the node) is the Xor of all the current close group
members of the current node. All close group members will recognise the group of this node and this
node will also know the close group of all of it's close nodes.

The `blockIdentifier` that represents a data item contains the hash of that data item. This allows
the `DataChain` to hold identifiers to data that can validate the data itself. This allows the data
to be republished as certain to have been created on the network itself.

To ensure there are no extension attacks possible the data size should also be maintained along
with any other identifying fields deemed required by the implementation. Additionally an HMAC can
be used to further secure the data in question.

## Block

A [Block][2] is made up of a [BlockIdentifier] and a vector of `PublicKey` and `Signature`.This vector
is known as the proof. Each proof tuple can be used to verify the signature is that of the
`BlockIdentifier` and that the `PublicKey` is the one used to sign this. These proofs are in fact
[NodeBlockProof]s

A link block has the same `proof` vector. This block type is the glue that holds the chain together
and provides the link of proofs right up until the current group can be identified. It is this
pattern that allow a series of links to be cryptographically secured. As each link is only valid if
signed by the majority of the previous (valid) link then a detectable series is calculable.

Blocks that have data as their `BlockIdentifer` part are merely slotted into the appropriate gap
between links. A block of data is validated in the same manner as the connections between links.

The last valid link can also be tested to contain a majority of the current close group. In this
case the chain is valid right to the last link. This phenomenon allows all blocks to be shown to be
valid.


## NodeBlock

A [NodeBlock] consists of the [BlockIdentifier] and the [NodeBlockProof]. Nodes will create these
and send them as messages to group members when the network mutates. This will require that for
every `Put` `Delete` or `Post` a new [BlockIdentifier] for that data item is created and sent to
all group members. The proof is this nodes `PublicKey` and `Signature`, allowing the receiving node
to call the `DataChain`'s `fn add_nodeblock()` to try and add this to the data chain.

In times of network churn a node will create a separate `LinkDescriptor` to create the
`BlockIdentifier` for this nodeblock. This linkdescriptor is s created by calling the
[create_link_descriptor()] method and passing the close_group **to that node** as the input. Each
node in the group will do the same and send the `NodeBlock` to that node.

This continual updating of the chain also provides a history of the network, both in terms of data
and also groups. Each block will contain a list of the nodes that have been seen on the network as
the chain evolved.

## Chain

The chain itself is a very simple vector of blocks. The [API] of the `DataChain` allows for
splitting, merging and validating chains. This allows chains to be exchanged and validated between
nodes. If a chain can be proven to be able to be owned (by calling the chain validate_ownership
function) by a receiving node then it is considered fully valid.

An interesting aspect though is the ability to "validate in history". This means that even if a
chain cannot be proven to be able to be fully published to a group (as there are not enough
remaining members of the group existing) it may still be queried with a few more conditions.

1. The current receiving node, did exist in the chain and has previously signed a block. Even
though others do not remain this node does believe the chain, but cannot prove it to anyone else.

2. A chain may contain an older link that is validate-able as there is a common link in a current
held chain and the published one. The published chain may hold data after this point that cannot be
validated, however the data up to the point of a common link (a link that holds enough common nodes
to provide a majority against a link we already know in our own chain) can be proven valid. This
phenomenon allows even older data than we know to be collected and increase the length of the
current chain. This allows the adding of "history" to an existing chain.



## Routing requirements

1. A node address will be a cryptographic signing key.

2. A node will attempt to join a previous group with the last known key. It will not though, join
the routing table at that stage. Routing will ask the upper layer (vaults in this case) if that
node is acceptable. While this process is taking place this joining node will be added to a list of
nodes attempting to join. If vaults agree the node is OK then routing will add this node to the
routing table.

3. If vaults reject a node, then it will follow the normal joining process (secure join)

## Vault requirements

1. A vault will allow majority - 1 nodes to join via the mechanism above.

2. On receiving a join request for a node (from routing), vaults will request the nodes `DataChain`

3. If this nodes `DataChain` is longer than an existing majority - 1 nodes, then nodes query the
joining node for data from the chain and then it is allowed to join.

4. All nodes that can hold a lot of data will try and build their data chain from existing nodes
holding such data (`Archive Nodes`). This data is transferred with the lowest priority.

5. On a churn even a node that is off line for a period may find on restart an existing node did
build a chain and now this restarting node has to join another group to begin the process again of
building a data chain.

Nodes will build their chains to become more valuable to the network and therefore earn more
safecoin. This process will encourage high capability nodes to spread evenly across the network.

Lower capability nodes will not attempt to build data history and will therefore have less earning
potential. This is perfectly good though and possibly a requirement of such a network.


# Additional observations

## Archive nodes

Nodes that hold the longest `DataChains` may be considered to be archive nodes. Such nodes will be
responsible for maintaining all network data for specific areas of the network address range. There
will be 3 archive nodes per group. These more reliable nodes have a vote weight of 2 within a group
and it would therefore require a minimum of 3 groups of archive nodes to collude against the
network. It is important to note that each group is chosen at random by the network.

### Archive node Datachain length

The length of the `DataChain` should be as long as possible. Although a node may not require to hold
data outwith it's current close group. It is prudent such nodes hold as much of the Chain as
possible as this all allow quicker rebuild of a network on complete outage. Nodes may keep such
structures and associated data in a container that prunes older blocks to make space for new blocks
as new blocks appear (FIFO or first in first out).

#### Additional requirements of Archive nodes

If an archive node requests data that is outwith its current close group, it should receive a higher
reward than usual.## Non Archive nodes

All nodes in a group will build on their `DataChain`, whether an Archive node or simply attempting
to become an archive node. Small nodes with little resources though may find it difficult to create
a `DataChain`of any significance. In these cases these smaller less capable nodes will receive
limited rewards as they do not have the ability to respond to many data retrieval requests, if any
at all. These small nodes though are still beneficial to the network to provide connectivity and
lower level consensus at the routing level.

A non archive node can request old data from existing archive nodes in a group, but the rate should
be limited in cases where there are already three such nodes in a group. These messages will be the
lowest priority messages in the group. Thereby any attacker will require to become an archive node
and this will take time, unless the group falls below three (or (group_size / 2) - 1 archive nodes
in which case the priority is increased on such relocation messages.

## Chained chains

As chains grow and nodes hold longer chains across many disparate groups, there will be commonalties
on `DataBlocks` held. Such links across chains has not as yet been fully analysed, however, it is
speculated that the ability to cross reference will enable a fuller picture of network data to be
built up.

### Structured data first version

To strengthen the validity of mutable data (StructuredData) the first version (version 0) may be
maintained in the chain. This will show age of such data, which may be particularly useful in types
of mutable data that do not change ownership or indeed where network created elements (such as any
currency) can be further validated.

## Archive node pointers

The possibility for a group to not have an ability, even with Archive nodes to store all data may
still exist in small imbalanced networks. Such groups may be able to delegate responsibility to
known larger nodes outwith their group, by passing data and also passing a `DtaChain` to prove
validity. This can introduce an addition to the `DataChain` object to provide pointers to data. In
such cases the larger nodes should receive a proportion of any reward for doing so. It is, however,
doubtful this particular paradigm will have to be enforced if possible archive nodes are pushed
across groups as described above.

###Network "difficulty"

The distance of the furthest group member to a nodes own ID is regarded as network difficulty. In
small networks this will wildly fluctuate. This value must be written to the nodes configuration
file, in case of SAFE this is the vault configuration file.

## Network restart / mass segmentation

The process described above will mean that decentralised network, far from potentially losing data
on restart should recover with a very high degree of certainty.

If a node restarts or in cases of massive churn there will be a significant reduction in network
difficulty. This reduction will mean that any nodes joining `again` should be accepted, regardless
of chain length.

If a restart has been detected, any node recognised in the last link of the chain will be allowed
entry again.


# Drawbacks

- In very small networks (less than approx 3000) network difficulty is a fluctuating number, this can
probably not be prevented, but may allow unwanted data or in fact prevent valid data from being
refreshed.
- This pattern is at it's earliest of stages and will require significant testing to ensure integrity of data as well as safety.
- Chain merging and data integrity checking is not well defined in this RFC and will require further analysis during implementation.


# Alternatives

None as of yet

# Unresolved questions

To be discovered during review.

[1]: https://dirvine.github.io/data_chain/master/data_chain/block_identifier/index.html
[2]: https://dirvine.github.io/data_chain/master/data_chain/block/struct.Block.html
[NodeBlock]: https://dirvine.github.io/data_chain/master/data_chain/node_block/struct.NodeBlock.html
[NodeBlockProof]: https://dirvine.github.io/data_chain/master/data_chain/node_block/struct.NodeBlockProof.html
[BlockIdentifier]: https://dirvine.github.io/data_chain/master/data_chain/block_identifier/enum.BlockIdentifier.html
[create_link_descriptor()]: https://dirvine.github.io/data_chain/master/data_chain/node_block/fn.create_link_descriptor.html
