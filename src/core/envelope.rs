#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyStatus {
    Passthrough,
    Filtered,
    Unsafe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEnvelope {
    pub filter_name: String,
    pub raw_id: Option<String>,
    pub bytes_before: usize,
    pub bytes_after: usize,
    pub omissions_count: usize,
    pub safety_status: SafetyStatus,
}

impl OutputEnvelope {
    pub fn passthrough(bytes: usize) -> Self {
        Self {
            filter_name: "passthrough".to_string(),
            raw_id: None,
            bytes_before: bytes,
            bytes_after: bytes,
            omissions_count: 0,
            safety_status: SafetyStatus::Passthrough,
        }
    }

    pub fn footer(&self) -> String {
        let raw_id = self.raw_id.as_deref().unwrap_or("none");
        format!(
            "\n[tss] filter={} raw={} bytes={}/{} omissions={} safety={:?}\n",
            self.filter_name,
            raw_id,
            self.bytes_after,
            self.bytes_before,
            self.omissions_count,
            self.safety_status
        )
    }
}
