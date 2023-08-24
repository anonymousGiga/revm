use crate::time::convert_to_nanoseconds;
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RevmMetricRecord {
    pub instruction_exec_time_records: Vec<(u8, u64)>,
    pub total_host_spend_time: u128,
    pub sload_hit_counter: u64,
    pub sload_not_hit_counter: u64,
    pub additional_overhead: Vec<u64>,
}

impl RevmMetricRecord {
    pub fn convert_cycles_to_ns(&mut self, cpu_frequency: f64) {
        self.additional_overhead
            .iter_mut()
            .for_each(|value| *value = convert_to_nanoseconds(*value, cpu_frequency));
    }

    pub fn not_empty(&self) -> bool {
        if !self.instruction_exec_time_records.is_empty()
            || !self.additional_overhead.is_empty()
            || self.sload_hit_counter != 0
            || self.sload_not_hit_counter != 0
        {
            return true;
        }
        false
    }
}
