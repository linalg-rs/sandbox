use sparse_traits::{IndexSet, IndexType};

pub struct DistributedIndexSet {
    ranges: Vec<Option<(IndexType, IndexType)>>,
    my_rank: IndexType,
    number_of_global_indices: IndexType,
}

impl DistributedIndexSet {
    pub fn new(range: (IndexType, IndexType), comm: &dyn mpi::traits::Communicator) -> Self {
        let comm_size = comm.size() as IndexType;
        let my_rank = comm.rank() as IndexType;

        let mut ranges = Vec::<Option<(IndexType, IndexType)>>::with_capacity(comm_size as usize);

        // The following code computes what index is on what rank. No MPI operation necessary.
        // Each process computes it from its own rank and the number of MPI processes in
        // the communicator

        let number_of_global_indices = range.1 - range.0;

        if number_of_global_indices <= comm_size {
            // If we have fewer indices than ranks simply
            // give one index to each rank until filled up.
            // Then fill the rest with None.
            for index in range.0..range.1 {
                ranges.push(Some((index, index + 1)));
            }
            for _ in range.1..comm_size {
                ranges.push(None);
            }
        } else {
            // We want to equally distribute the range
            // among the ranks. Assume that we have 12
            // indices and want to distribute among 5 ranks.
            // Then each rank gets 12 / 5 = 2 indices. However,
            // we have a remainder 12 % 5 = 2. Those two indices
            // are distributed among the first two ranks. So at
            // the end we have the distribution
            // 0 -> (0, 3)
            // 1 -> (3, 6)
            // 2 -> (6, 8)
            // 3 -> (8, 10)
            // 4 -> (10, 12)

            let chunk = number_of_global_indices / comm_size;
            let remainder = number_of_global_indices % comm_size;
            let mut count = range.0;
            let mut new_count;

            for index in 0..number_of_global_indices {
                if index < remainder {
                    // Add one remainder index to the first
                    // indices.
                    new_count = count + chunk + 1;
                } else {
                    // When the remainder is used up just
                    // add chunk size indices to each rank.
                    new_count = count + chunk;
                }
                ranges.push(Some((count, new_count)));
                count = new_count;
            }
        }

        Self {
            ranges,
            my_rank,
            number_of_global_indices,
        }
    }
}

impl IndexSet for DistributedIndexSet {
    fn index_range(&self, rank: IndexType) -> &Option<(IndexType, IndexType)> {
        self.ranges.get(rank).unwrap()
    }

    fn local_range(&self) -> &Option<(IndexType, IndexType)> {
        self.ranges.get(self.my_rank).unwrap()
    }

    fn number_of_local_indices(&self) -> IndexType {
        if let &Some((first, last)) = self.local_range() {
            last - first
        } else {
            0
        }
    }

    fn number_of_global_indices(&self) -> IndexType {
        self.number_of_global_indices
    }
}
