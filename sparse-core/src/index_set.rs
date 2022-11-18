use sparse_traits::{IndexSet, IndexType};

pub struct SerialIndexSet {
    range: (IndexType, IndexType),
}

pub struct DistributedIndexSet {
    // ranges stores a mapping between rank and
    // the local range of indices on that process.
    // Imagine we have 10 indices (from 0 to 9) and
    // 3 processes.
    // We would then have
    // ranges[0] = Some((0, 3)),
    // ranges[1] = Some((3, 6)),
    // ranges[2] = Some((6, 10))
    // If my rank is 2. Then my local range is (6, 10)
    // If we have 5 indices and 6 processes then not
    // every process gets a range and ranges[5] = None
    ranges: Vec<Option<(IndexType, IndexType)>>,
    my_rank: IndexType,
}

impl SerialIndexSet {
    pub fn new(range: (IndexType, IndexType)) -> Self {
        SerialIndexSet { range }
    }
}

impl IndexSet for SerialIndexSet {
    fn number_of_local_indices(&self) -> IndexType {
        self.number_of_global_indices()
    }

    fn local_range(&self) -> (IndexType, IndexType) {
        self.range
    }

    fn number_of_global_indices(&self) -> IndexType {
        self.range.1 - self.range.0
    }

    fn index_range(&self, rank: IndexType) -> Option<(IndexType, IndexType)> {
        match rank {
            0 => Some(self.range),
            _ => None,
        }
    }
}

impl DistributedIndexSet {
    fn new(range: (IndexType, IndexType), comm: &dyn mpi::traits::Communicator) -> Self {
        let ranges = Vec::<(IndexType, IndexType)>::new();
        let my_rank = comm.rank() as IndexType;

        // The following code computes what index is on what rank. No MPI operation necessary.
        // Each process computes it from its own rank and the number of MPI processes in
        // the communicator

        Self { ranges, my_rank }
    }
}
