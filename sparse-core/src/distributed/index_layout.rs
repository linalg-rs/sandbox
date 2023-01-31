use crate::local::index_layout::LocalIndexLayout;
use mpi::traits::Communicator;
use mpi::Count;
use sparse_traits::{IndexLayout, IndexType};

pub struct DistributedIndexLayout<'a, C: Communicator> {
    ranges: Vec<Option<(IndexType, IndexType)>>,
    global_range: (IndexType, IndexType),
    my_rank: IndexType,
    local_layout: Option<LocalIndexLayout>,
    number_of_global_indices: IndexType,
    counts: Vec<Count>,
    displacements: Vec<Count>,
    comm: &'a C,
}

impl<'a, C: Communicator> DistributedIndexLayout<'a, C> {
    pub fn new(range: (IndexType, IndexType), comm: &'a C) -> Self {
        let comm_size = comm.size() as IndexType;
        let my_rank = comm.rank() as IndexType;
        let mut counts = vec![0 as Count; comm_size as usize];
        let mut displacements =  vec![0 as Count; comm_size as usize];

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

            for index in 0..number_of_global_indices {
                counts[index] = 1;
                displacements[index] = index as Count;
            }

            for index in number_of_global_indices..comm_size {
                counts[index] = 0;
                displacements[index] = number_of_global_indices as Count;
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


            for index in 0..comm_size {
                if index < remainder {
                    // Add one remainder index to the first
                    // indices.
                    new_count = count + chunk + 1;
                } else {
                    // When the remainder is used up just
                    // add chunk size indices to each rank.
                    new_count = count + chunk;
                }
                counts[index] = (new_count - count) as Count;
                displacements[index] = count as Count;
                ranges.push(Some((count, new_count)));
                count = new_count;
            }
        }

        let local_layout = match ranges.get(my_rank).unwrap() {
            Some(range) => Some(LocalIndexLayout::new((0, range.1 - range.0))),
            None => None,
        };
        Self {
            ranges,
            global_range: range,
            my_rank,
            local_layout,
            number_of_global_indices,
            counts,
            displacements,
            comm,
        }
    }
    pub fn comm(&self) -> &C {
        self.comm
    }

    pub fn counts(&self) -> &Vec<Count> {
        &self.counts
    }

    pub fn displacements(&self) -> &Vec<Count> {
        &self.displacements
    }

    // This method is needed for Distributed vectors to obtain a dummy layout
    // for the local vector.
    pub(crate) fn local_layout(&self) -> Option<&LocalIndexLayout> {
        self.local_layout.as_ref()
    }

    pub fn is_same(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<'a, C: Communicator> IndexLayout for DistributedIndexLayout<'a, C> {
    fn index_range(&self, rank: IndexType) -> &Option<(IndexType, IndexType)> {
        assert!(
            rank < self.comm.size() as IndexType,
            "No rank with index {} exists.",
            rank
        );
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

    fn global_range(&self) -> &(IndexType, IndexType) {
        &self.global_range
    }

    fn map(&self, index: IndexType) -> Option<IndexType> {
        if index < self.number_of_local_indices() {
            Some(self.local_range().unwrap().0 + index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use mpi;

    #[test]
    fn test_distributed_index_set() {
        let universe = mpi::initialize().unwrap();
        let world = universe.world();

        let index_layout = DistributedIndexLayout::new((3, 14), &world);

        // Test that the range is correct on rank 0
        assert_eq!(index_layout.index_range(0).unwrap(), (3, 14));

        // Test that the number of global indices is correct.
        assert_eq!(index_layout.number_of_global_indices(), 11);

        // Test that map works

        assert_eq!(index_layout.map(2).unwrap(), 5);
    }
}
