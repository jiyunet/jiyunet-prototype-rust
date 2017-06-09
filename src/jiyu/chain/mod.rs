struct Block {
    artifacts: Vec<ArtifactEntry>
}

impl Entity for Block {
    unimplemented!();
}

struct ArtifactEntry {
    sig: [u8; 32]
    data: Artifact
}

struct Artifact {
    timestamp: u64,
    owner: Address,
    data: ArtifactData
}

enum ArtifactData {
    Unknown { ver: u16, data: [u8] }
}
