pub struct LineData {
    status: String,
    continuous_broken_lengths: Vec<usize>,
}

impl LineData {
    pub fn get_status<'a>(&'a self) -> &'a String {
        return &self.status;
    }

    pub fn get_continuous_broken_lengths<'a>(&'a self) -> &'a Vec<usize> {
        return &self.continuous_broken_lengths;
    }

    pub fn new(status: String, continuous_broken_lengths: Vec<usize>) -> Self {
        return Self {status, continuous_broken_lengths};
    }
}