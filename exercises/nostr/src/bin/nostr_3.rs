impl Event {
    fn compute_id(&self) -> String {
        use sha2::{Sha256, Digest};
        use serde_json::json;

        let serialized = json!([
            0,
            self.pubkey,
            self.created_at,
            self.kind,
            self.tags,
            self.content
        ]);

        let mut hasher = Sha256::new();
        hasher.update(serialized.to_string());
        format!("{:x}", hasher.finalize())
    }
}

impl Event {
    fn sign(&mut self) {
        self.sig = "fake_signature_for_now".to_string();
    }
}

